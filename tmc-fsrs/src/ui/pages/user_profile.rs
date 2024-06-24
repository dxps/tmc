use dioxus::prelude::*;

use crate::server::UserAccount;
use crate::ui::comps::{render_go_to_login, Nav, NavProps};
use crate::ui::{State, APP_READY};

#[component]
pub fn UserProfile(username: String) -> Element {
    //
    if *APP_READY.read() == false {
        return rsx! {};
    };
    let state = use_context::<Signal<State>>();
    if state().current_user.is_none() {
        log::debug!(">>> [UserProfile] There is no locally saved user.");
        render_go_to_login()
    } else {
        render_user_profile_form(state().current_user.unwrap())
    }
}

fn render_user_profile_form(ua: UserAccount) -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav { active_path: NavProps::users_section() }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-6 mt-24 mb-8",
                    h1 { class: "text-3xl text-[#333] font-bold text-center", "User Profile" }
                    div { class: "mt-8 space-y-6",
                        div {
                            label { class: "text-sm block mb-2", "Username" }
                            input {
                                class: "w-full",
                                r#type: "text",
                                placeholder: "Username",
                                value: "{ua.username}",
                                maxlength: 48
                            }
                        }
                        div {
                            label { class: "text-sm block mb-2", "Email" }
                            input {
                                class: "w-full rounded-md py-2.5",
                                r#type: "text",
                                placeholder: "Email",
                                value: "{ua.email}",
                                maxlength: 64
                            }
                        }
                        div {
                            label { class: "text-sm block mb-2", "Biography" }
                            textarea {
                                class: "w-full rounded-md py-2.5 px-4",
                                cols: 64,
                                rows: 8,
                                placeholder: "Biography",
                                value: "{ua.bio}",
                                maxlength: 1024
                            }
                        }
                        div { class: "text-center my-8",
                            button {
                                class: "bg-blue-50 hover:bg-blue-100 drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| { async move {} },
                                "Save"
                            }
                        }
                    }
                }
            }
        }
    }
}
