use dioxus::prelude::*;

use crate::{
    server::fns::auth::login,
    ui::comps::{Nav, NavProps},
};
use crate::ui::routes::Route;

#[component]
pub fn Login() -> Element {
    //
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let nav = use_navigator();

    rsx! {
        div {
            class: "flex flex-col min-h-screen bg-gray-100",
            Nav { active_path: NavProps::login() },
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-6",
                    div { class: "text-xl mb-6 px-2 text-center text-gray-600",
                        "Login to your account"
                    }
                    div { class: "mt-4 space-y-4",
                        div {
                            input {
                                class: "px-3 py-3 rounded-lg outline-none border-2 focus:border-green-300",
                                name: "email", r#type: "email", placeholder: "Email address",
                                value: "{email}",
                                autofocus: "true",
                                oninput: move |evt| { email.set(evt.value()); },
                                onmounted: move |evt| async move {
                                    log::debug!(">>> [Login] email input mounted. Setting the focus on it.");
                                    _ = evt.set_focus(true).await;
                                }
                            }
                        }
                        div {
                            input {
                                class: "px-3 py-3 rounded-lg outline-none border-2 focus:border-green-300",
                                name: "password", r#type: "password", placeholder: "Password",
                                value: "{password}",
                                oninput: move |e| { password.set(e.value()); }
                            }
                        }
                        div { class: "justify-center text-center my-8",
                            button {
                                class: "bg-blue-50 hover:bg-blue-100 drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| {
                                    async move {
                                        match login(format!("{}", email), format!("{}", password)).await {
                                            Ok(account) => {
                                                log::debug!(">>> [Login] Authenticated and got {:?}. Going to home.", account);
                                                nav.push(Route::Home {});
                                            }
                                            Err(e) => {
                                                log::debug!(">>> [Login] Authentication failed. Error: {}", e);
                                                // TODO: Show an error in UI.
                                            }
                                        }
                                    }
                                },
                                "Login"
                            }
                        }
                    }
                }
            }
        }
    }
}
