use dioxus::prelude::*;

use crate::ui::{
    comps::{Nav, NavProps},
    routes::Route,
};

#[component]
pub fn UserProfile(username: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen bg-gray-100",
            Nav { active_path: NavProps::blog() },
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3",
                    div { class: "text-xl px-2",
                        "User profile page for {username}"
                    }
                }
                div { class: "pt-8",
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}
