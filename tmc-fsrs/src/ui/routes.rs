use dioxus::prelude::*;

use crate::ui::pages::{Blog, Home, Sample};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    Blog { id: i32 },

    #[route("/sample")]
    Sample {},
}
