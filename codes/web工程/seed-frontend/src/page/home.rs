use super::ViewPage;
use crate::{
    entity::{
        Viewer,
    },
    page, GMsg, Session,
};
use seed::prelude::*;
use std::future::Future;

// ------ ------
//     Model
// ------ ------

// ------ Model ------

#[derive(Default)]
pub struct Model {
    session: Session,
}

impl Model {
    pub const fn session(&self) -> &Session {
        &self.session
    }
}

impl From<Model> for Session {
    fn from(model: Model) -> Self {
        model.session
    }
}

// ------ Status ------

enum Status<T> {
    Loading,
    LoadingSlowly,
    Loaded(T),
    Failed,
}

impl<T> Default for Status<T> {
    fn default() -> Self {
        Self::Loading
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(session: Session, _: &mut impl Orders<Msg, GMsg>) -> Model {
    Model {
        session,
    }
}

// ------ ------
//     Sink
// ------ ------

pub fn sink(g_msg: GMsg, model: &mut Model) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
        }
        _ => (),
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
#[allow(clippy::pub_enum_variant_names)]
pub enum Msg {
    SlowLoadThresholdPassed,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::SlowLoadThresholdPassed => {
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("test", view_content(model))
}

// ====== PRIVATE ======

fn view_content(model: &Model) -> Node<Msg> {
    div![class!["home-page"], view_banner(), ]
}

fn view_banner() -> Node<Msg> {
    div![
        class!["banner"],
        div![
            class!["container"],
            h1![class!["logo-font"], "Rust DFS"],
            p!["a log-test page based on realworld app"]
        ]
    ]
}