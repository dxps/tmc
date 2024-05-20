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

    /// The catch all route, including the placement of the URL path segments in the `route` property.
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    //
    let path: String = route.iter().map(|elem| format!("/{}", *elem)).collect();
    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 text-center",
                    h1 { class: "text-xl", "Page not found" }
                    p { class: "py-4", "We are terribly sorry, but the page you requested doesn't exist." }
                    pre { class: "text-xs text-slate-600", "Unknown path {path}" }
                }
                div { class: "pt-8",
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}
