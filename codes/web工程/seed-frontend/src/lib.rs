#![allow(
    clippy::single_match,
    clippy::large_enum_variant,
    clippy::must_use_candidate
)]
#![allow(clippy::default_trait_access)] // because of problem with `strum_macros::EnumIter`

#[macro_use]
extern crate seed;
use entity::username;
use helper::take;
use seed::prelude::*;
use std::convert::TryInto;

pub use route::Route;
pub use session::Session;

mod coder;
mod entity;
mod helper;
mod loading;
mod logger;
mod page;
mod request;
mod route;
mod session;
mod storage;

// ------ ------
//     Model
// ------ ------

enum Model {
    Redirect(Session),
    NotFound(Session),
    Home(page::home::Model),
    //Settings(page::settings::Model),
    Login(page::login::Model),
    Register(page::register::Model),
    //Profile(page::profile::Model<'a>, username::Username<'a>),
}

impl Default for Model {
    fn default() -> Self {
        Model::Redirect(Session::default())
    }
}

impl From<Model> for Session {
    fn from(model: Model) -> Self {
        use Model::*;
        match model {
            Redirect(session) | NotFound(session) => session,
            Home(model) => model.into(),
            Login(model) => model.into(),
            Register(model) => model.into(),
        }
    }
}

// ------ ------
// Before Mount
// ------ ------

fn before_mount(_: Url) -> BeforeMount {
    // Since we have the "loading..." text in the app section of index.html,
    // we use MountType::Takover which will overwrite it with the seed generated html
    BeforeMount::new().mount_type(MountType::Takeover)
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(
    url: Url,
    orders: &mut impl Orders<Msg, GMsg>,
) -> AfterMount<Model> {
    orders.send_msg(Msg::RouteChanged(url.try_into().ok()));

    let model = Model::Redirect(Session::new(storage::load_viewer()));
    AfterMount::new(model).url_handling(UrlHandling::None)
}

// ------ ------
//     Sink
// ------ ------

pub enum GMsg {
    RoutePushed(Route),
    SessionChanged(Session),
}

fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    if let GMsg::RoutePushed(ref route) = g_msg {
        orders.send_msg(Msg::RouteChanged(Some(route.clone())));
    }

    match model {
        Model::NotFound(_) | Model::Redirect(_) => {
            if let GMsg::SessionChanged(session) = g_msg {
                *model = Model::Redirect(session);
                route::go_to(Route::Home, orders);
            }
        }
        Model::Home(model) => {
            page::home::sink(g_msg, model);
        }
        Model::Login(model) => {
            page::login::sink(g_msg, model, &mut orders.proxy(Msg::LoginMsg));
        }
        Model::Register(model) => {
            page::register::sink(g_msg, model, &mut orders.proxy(Msg::RegisterMsg));
        }
    }
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::enum_variant_names)]
enum Msg {
    RouteChanged(Option<Route>),
    HomeMsg(page::home::Msg),
    LoginMsg(page::login::Msg),
    RegisterMsg(page::register::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::RouteChanged(route) => {
            change_model_by_route(route, model, orders);
        }
        Msg::HomeMsg(module_msg) => {
            if let Model::Home(module_model) = model {
                page::home::update(module_msg, module_model, &mut orders.proxy(Msg::HomeMsg));
            }
        }
        Msg::LoginMsg(module_msg) => {
            if let Model::Login(module_model) = model {
                page::login::update(module_msg, module_model, &mut orders.proxy(Msg::LoginMsg));
            }
        }
        Msg::RegisterMsg(module_msg) => {
            if let Model::Register(module_model) = model {
                page::register::update(
                    module_msg,
                    module_model,
                    &mut orders.proxy(Msg::RegisterMsg),
                );
            }
        }
    }
}

fn change_model_by_route(
    route: Option<Route>,
    model: &mut Model,
    orders: &mut impl Orders<Msg, GMsg>,
) {
    let mut session = || Session::from(take(model));
    match route {
        None => *model = Model::NotFound(session()),
        Some(route) => match route {
            Route::Root => route::go_to(Route::Home, orders),
            Route::Logout => {
                storage::delete_app_data();
                orders.send_g_msg(GMsg::SessionChanged(Session::Guest));
                route::go_to(Route::Home, orders)
            }
            Route::Home => {
                *model = Model::Home(page::home::init(session(), &mut orders.proxy(Msg::HomeMsg)));
            }
            Route::Login => {
                *model = Model::Login(page::login::init(session()));
            }
            Route::Register => {
                *model = Model::Register(page::register::init(session()));
            }
        },
    };
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    use page::Page;
    match model {
        Model::Redirect(session) => Page::Other.view(page::blank::view(), session.viewer()),
        Model::NotFound(session) => Page::Other.view(page::not_found::view(), session.viewer()),
        Model::Home(model) => Page::Home
            .view(page::home::view(model), model.session().viewer())
            .map_msg(Msg::HomeMsg),
        Model::Login(model) => Page::Login
            .view(page::login::view(model), model.session().viewer())
            .map_msg(Msg::LoginMsg),
        Model::Register(model) => Page::Register
            .view(page::register::view(model), model.session().viewer())
            .map_msg(Msg::RegisterMsg),
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(|url| Some(Msg::RouteChanged(url.try_into().ok())))
        .sink(sink)
        .build_and_start();
}
