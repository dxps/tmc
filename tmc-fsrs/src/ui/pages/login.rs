use dioxus::prelude::*;

use crate::{
    server::fns::auth::login,
    ui::comps::{Nav, NavProps},
};

#[component]
pub fn Login() -> Element {
    //
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

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
                                class: "px-3 py-3 rounded-lg outline-none border-2 focus:border-green-500",
                                name: "email", r#type: "email", placeholder: "Email address",
                                value: "{email}",
                                oninput: move |evt| { email.set(evt.value()); }
                            }
                        }
                        div {
                            input {
                                class: "px-3 py-3 rounded-lg outline-none border-2 focus:border-green-500",
                                name: "password", r#type: "password", placeholder: "Password",
                                value: "{password}",
                                oninput: move |evt| { password.set(evt.value()); }
                            }
                        }
                        div { class: "justify-center text-center my-8",
                            button {
                                class: "bg-blue-50 hover:bg-blue-100 drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| {
                                    async move {login(format!("{}", email), format!("{}", password)).await.unwrap(); }
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
