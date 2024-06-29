use dioxus::prelude::*;

use crate::server::fns::user_profile::save_user_profile_primary_info;
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
        render_user_profile_page(username, state().current_user.unwrap())
    }
}

fn render_user_profile_page(username: String, ua: UserAccount) -> Element {
    //
    log::debug!(">>> [render_user_profile_page] Username: {}, UserAccount: {:?}", username, ua);
    let mut tab_to_show = use_signal(|| "primary_info".to_string());

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav { active_path: NavProps::users_section() }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-6 mt-24 mb-8",
                    h1 { class: "text-3xl text-[#333] font-bold text-center",
                        if username == ua.username {
                            { "My Profile" }
                        } else {
                            { format!("{}'s Profile", username) }
                        }
                    }
                    // The tabs.
                    ul { class: "flex gap-4 bg-gray-100 rounded-lg my-4 p-[3.4px] w-max overflow-hidden font-sans mx-auto",
                        li {
                            class: "text-green-600 rounded-lg font-semibold text-center text-sm bg-white py-2 px-4 tracking-wide cursor-pointer",
                            onclick: move |_| tab_to_show.set("primary_info".to_string()),
                            "Primary Info"
                        }
                        li {
                            class: "text-gray-600 rounded-lg text-center text-sm hover:bg-white hover:text-lilac py-2 px-4 tracking-wide cursor-pointer",
                            onclick: move |_| tab_to_show.set("security".to_string()),
                            "Security"
                        }
                    }
                    if tab_to_show() == "primary_info".to_string() {
                        PrimaryInfo { ua }
                    } else if tab_to_show() == "security".to_string() {
                        Security { ua }
                    }
                }
            }
        }
    }
}

#[component]
fn PrimaryInfo(ua: UserAccount) -> Element {
    //
    let mut username = use_signal(|| ua.username.clone());
    let mut email = use_signal(|| ua.email.clone());
    let mut bio = use_signal(|| ua.bio.clone());
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let mut saved = use_signal(|| false);

    rsx! {
        div { class: "mt-8 space-y-6",
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Username" }
                input {
                    class: "w-full",
                    r#type: "text",
                    placeholder: "Username",
                    value: "{ua.username}",
                    maxlength: 48,
                    oninput: move |evt| { username.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Email" }
                input {
                    class: "w-full rounded-md py-2.5",
                    r#type: "text",
                    placeholder: "Email",
                    value: "{ua.email}",
                    maxlength: 64,
                    oninput: move |evt| { email.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Biography" }
                textarea {
                    class: "w-full rounded-md py-2.5 px-3",
                    cols: 64,
                    rows: 6,
                    placeholder: "Biography",
                    value: "{ua.bio}",
                    maxlength: 1024,
                    oninput: move |evt| { bio.set(evt.value()) }
                }
            }
            div { class: "text-center my-8",
                button {
                    class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 py-2 rounded-md",
                    onclick: move |_| {
                        let mut ua = ua.clone();
                        async move {
                            match save_user_profile_primary_info(
                                    ua.id.clone(),
                                    username(),
                                    email(),
                                    bio(),
                                )
                                .await
                            {
                                Ok(_) => {
                                    err.set(None);
                                    saved.set(true);
                                    ua.username = username();
                                    ua.email = email();
                                    ua.bio = bio();
                                    let new_state = State::new(&ua);
                                    new_state.save_to_localstorage();
                                    let mut state = use_context::<Signal<State>>();
                                    state.write().clone_from(&new_state);
                                }
                                Err(e) => {
                                    err.set(Some(e.to_string()));
                                    saved.set(false);
                                }
                            }
                        }
                    },
                    "Save"
                }
            }
            if err().is_some() {
                div { class: "text-center text-red-600 my-8",
                    span { {err().unwrap()} }
                }
            } else if saved() {
                div { class: "text-center text-green-600 my-8",
                    span { { "Successfully saved" } }
                }
            }
        }
    }
}

#[component]
fn Security(ua: UserAccount) -> Element {
    //
    let mut new_password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());

    rsx! {
        div { class: "mt-8 space-y-6",
            div { class: "flex flex-row text-sm text-gray-500",
                { "Id: " },
                { ua.id.clone() }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "New password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Set a new password",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { new_password.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "New password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Set a new password",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { confirm_password.set(evt.value()) }
                }
            }
        }
    }
}
