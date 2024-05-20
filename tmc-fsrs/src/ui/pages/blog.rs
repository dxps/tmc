use dioxus::prelude::*;

use crate::ui::{comps::Nav, routes::Route};

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3",
                    div { class: "text-xl px-2",
                        "Blog post {id}"
                    }
                }
                div { class: "pt-8",
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}
