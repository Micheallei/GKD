use std::{borrow::Cow, convert::TryFrom, fmt};

use seed::prelude::*;

use crate::{
    entity::Username,
    GMsg,
};

pub fn go_to<Ms: 'static>(route: Route, orders: &mut impl Orders<Ms, GMsg>) {
    seed::push_route(route.clone());
    orders.send_g_msg(GMsg::RoutePushed(route));
}

// ------ Route ------

#[derive(Clone, Debug)]
pub enum Route {
    Home,
    Root,
    Login,
    Logout,
    Register,
}

impl Route {
    pub fn path(&self) -> Vec<&str> {
        use Route::*;
        match self {
            Home | Root => vec![],
            Login => vec!["login"],
            Logout => vec!["logout"],
            Register => vec!["register"],
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl From<Route> for seed::Url {
    fn from(route: Route) -> Self {
        route.path().into()
    }
}

impl TryFrom<seed::Url> for Route {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Some(Route::Home),
            Some("login") => Some(Route::Login),
            Some("logout") => Some(Route::Logout),
            Some("register") => Some(Route::Register),
            _ => None,
        }
        .ok_or(())
    }
}

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::convert::TryInto;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn home_route_test() {
        // ====== ARRANGE ======
        let url = seed::Url::new(vec![""]);

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Home) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn home_route_trailing_slash_test() {
        // ====== ARRANGE ======
        let url = seed::Url::new(vec![""]);

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Home) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn login_route_test() {
        // ====== ARRANGE ======
        let url = seed::Url::new(vec!["login"]);

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Login) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn logout_route_test() {
        // ====== ARRANGE ======
        let url = seed::Url::new(vec!["logout"]);

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Logout) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn register_route_test() {
        // ====== ARRANGE ======
        let url = seed::Url::new(vec!["register"]);

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Register) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn invalid_route_test() {
        // ====== ARRANGE ======
        let url = seed::Url::new(vec!["unknown_url"]);

        // ====== ACT ======
        let route: Result<Route, ()> = url.try_into();

        // ====== ASSERT ======
        assert!(route.is_err())
    }
}
