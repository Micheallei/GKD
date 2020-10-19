use crate::{
    entity::{ErrorMessage, Username, Viewer},
    Route,
};
use seed::prelude::*;
use std::borrow::{Borrow, Cow};

pub mod blank;
pub mod home;
pub mod login;
pub mod not_found;
pub mod register;

pub fn scroll_to_top() {
    seed::window().scroll_to_with_scroll_to_options(
        web_sys::ScrollToOptions::new()
            .top(0.)
            .left(0.)
            .behavior(web_sys::ScrollBehavior::Smooth),
    )
}

pub fn view_errors<Ms: Clone>(dismiss_errors: Ms, errors: &[ErrorMessage]) -> Node<Ms> {
    if errors.is_empty() {
        empty![]
    } else {
        div![
            class!["error-messages"],
            style! {
                "position" => "fixed",
                "top" => 0,
                "background" => "rgb(250, 250, 250)",
                "padding" => "20px",
                "border" => "1px solid",
                "z-index" => 9999,
            },
            errors.iter().map(|error| p![error]),
            button![simple_ev(Ev::Click, dismiss_errors), "Ok"]
        ]
    }
}

// ------ ViewPage ------

#[allow(clippy::module_name_repetitions)]
pub struct ViewPage<'a, Ms: 'static> {
    title_prefix: Cow<'a, str>,
    content: Node<Ms>,
}

impl<'a, Ms> ViewPage<'a, Ms> {
    pub fn new(title_prefix: impl Into<Cow<'a, str>>, content: Node<Ms>) -> Self {
        Self {
            title_prefix: title_prefix.into(),
            content,
        }
    }
    pub fn title(&self) -> String {
        format!("{} - test", self.title_prefix)
    }
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_content(self) -> Node<Ms> {
        self.content
    }
}

// ------ Page ------

pub enum Page {
    Other,
    Home,
    Login,
    Register,
}

#[allow(clippy::unused_self)]
impl Page {
    fn is_active(&self, route: &Route) -> bool {
        match (self, route) {
            (Page::Home, Route::Home)
            | (Page::Login, Route::Login)
            | (Page::Register, Route::Register) => true,
            _ => false,
        }
    }

    // ------ view methods ------

    pub fn view<Ms>(&self, view_page: ViewPage<Ms>, viewer: Option<&Viewer>) -> Vec<Node<Ms>> {
        seed::document().set_title(&view_page.title());

        vec![
            self.view_header(viewer),
            view_page.into_content(),
            self.view_footer(),
        ]
    }

    // ====== PRIVATE ======

    fn view_header<Ms>(&self, viewer: Option<&Viewer>) -> Node<Ms> {
        nav![
            class!["navbar", "navbar-light"],
            div![
                class!["container"],
                a![
                    class!["navbar-brand"],
                    attrs! {At::Href => Route::Home.to_string()},
                    "test"
                ],
                ul![
                    class!["nav navbar-nav pull-xs-right"],
                    self.view_navbar_link(&Route::Home, "Home"),
                    self.view_menu(viewer),
                ],
            ]
        ]
    }

    fn view_footer<Ms>(&self) -> Node<Ms> {
        footer![div![
            class!["container"],
            a![
                class!["logo-font"],
                attrs! {At::Href => Route::Home.to_string()},
                "test"
            ],
        ]]
    }

    // ------ view_header helpers ------

    fn view_navbar_link<Ms>(&self, route: &Route, link_content: impl UpdateEl<El<Ms>>) -> Node<Ms> {
        li![
            class!["nav-item"],
            a![
                class![
                    "nav-link",
                    "active" => self.is_active(route),
                ],
                attrs! {At::Href => route.to_string()},
                link_content
            ]
        ]
    }

    fn view_menu<Ms>(&self, viewer: Option<&Viewer>) -> Vec<Node<Ms>> {
        match viewer {
            None => vec![
                self.view_navbar_link(&Route::Login, "Sign in"),
                self.view_navbar_link(&Route::Register, "Sign up"),
            ],
            Some(viewer) => vec![
                self.view_navbar_link(&Route::Logout, "Sign out"),
            ],
        }
    }
}
