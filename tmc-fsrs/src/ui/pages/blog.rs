use dioxus::prelude::*;

use crate::ui::routes::Route;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        "Blog post {id}"
        br {}
        Link { to: Route::Home {}, "Go to Home" }

    }
}
