use dioxus::prelude::*;

use crate::ui::comps::nav::common::{style_link, NavProps};
use crate::ui::routes::Route;
use crate::ui::State;

pub fn NavUserMenu(props: NavProps) -> Element {
    //
    let state = use_context::<Signal<State>>();
    let mut show_dropdown = use_signal(|| false);

    if state().current_user.is_none() {
        log::debug!(">>> [NavUserMenu] There is no locally saved user.");
        rsx! {
            Link {
                    class: style_link(&props.active_path, NavProps::login()).to_owned() + "hidden sm:inline-block sm:ml-auto sm:mr-3",
                    to: Route::Login {}, "Login",
            }
        }
    } else {
        let username = state().current_user.unwrap().username;
        log::debug!(
            ">>> [NavUserMenu] There is a locally saved user with username: {:?}.",
            username
        );
        rsx! {
            div {
                class: "flex flex-col items-end",
                // "UserMenu ({username})"
                button {
                    class: "px-4 py-1 align  rounded-lg text-[#333] text-sm outline-none hover:bg-gray-100",
                    onclick: move |_| {
                        let curr_val = show_dropdown();
                        *show_dropdown.write() = !curr_val;
                    },
                    div {
                        class: "rounded-full",
                        dangerous_inner_html: btn_user_icon(),
                    }
                }
                if show_dropdown() {
                    NavUserDropdown { username }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct NavUserDropdownProps {
    username: String,
}

fn NavUserDropdown(props: NavUserDropdownProps) -> Element {
    //
    rsx! {
        div {
            class: "w-48 mr-2 bg-white rounded-lg shadow-2xl",
            div {
                ul {
                    class: "absolute shadow-lg bg-white py-2 z-[1000] min-w-full w-max rounded-lg max-h-96 overflow-auto",
                    li {
                        class: "py-2.5 px-12 flex items-center text-[#888] text-sm",
                        "{props.username} user menu"
                    }
                    li {
                        class: "py-2.5 px-5 flex items-center hover:bg-gray-100 text-[#333] text-sm cursor-pointer",
                        div { dangerous_inner_html: user_icon() },
                        "My profile"
                    }
                    li {
                        class: "py-2.5 px-5 flex items-center hover:bg-gray-100 text-[#333] text-sm cursor-pointer",
                        onclick: move |_| { async move {
                                handle_logout().await;
                            }
                        },
                        div { dangerous_inner_html: logout_icon() },
                        "Logout"
                    }
                }
            }
        }
    }
}

async fn handle_logout() {
    use crate::server::fns::auth::logout;
    use crate::ui::State;

    log::debug!(">>> [NavUserDropdown] Logout clicked.");
    logout().await.unwrap();
    let state = State::default();
    state.save_to_localstorage();
    let mut state_sgnl = use_context::<Signal<State>>();
    *state_sgnl.write() = state;
    let nav = &use_navigator();
    nav.push(Route::Home {});
}

fn btn_user_icon() -> String {
    r#"
    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-5 h-5" viewBox="0 0 512 512">
        <path
        d="M337.711 241.3a16 16 0 0 0-11.461 3.988c-18.739 16.561-43.688 25.682-70.25 25.682s-51.511-9.121-70.25-25.683a16.007 16.007 0 0 0-11.461-3.988c-78.926 4.274-140.752 63.672-140.752 135.224v107.152C33.537 499.293 46.9 512 63.332 512h385.336c16.429 0 29.8-12.707 29.8-28.325V376.523c-.005-71.552-61.831-130.95-140.757-135.223zM446.463 480H65.537V376.523c0-52.739 45.359-96.888 104.351-102.8C193.75 292.63 224.055 302.97 256 302.97s62.25-10.34 86.112-29.245c58.992 5.91 104.351 50.059 104.351 102.8zM256 234.375a117.188 117.188 0 1 0-117.188-117.187A117.32 117.32 0 0 0 256 234.375zM256 32a85.188 85.188 0 1 1-85.188 85.188A85.284 85.284 0 0 1 256 32z"
        data-original="\#000000"></path>
    </svg>
    "#.to_string()
}

fn user_icon() -> String {
    r#"
    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-4 h-4 mr-3" viewBox="0 0 512 512">
        <path
        d="M337.711 241.3a16 16 0 0 0-11.461 3.988c-18.739 16.561-43.688 25.682-70.25 25.682s-51.511-9.121-70.25-25.683a16.007 16.007 0 0 0-11.461-3.988c-78.926 4.274-140.752 63.672-140.752 135.224v107.152C33.537 499.293 46.9 512 63.332 512h385.336c16.429 0 29.8-12.707 29.8-28.325V376.523c-.005-71.552-61.831-130.95-140.757-135.223zM446.463 480H65.537V376.523c0-52.739 45.359-96.888 104.351-102.8C193.75 292.63 224.055 302.97 256 302.97s62.25-10.34 86.112-29.245c58.992 5.91 104.351 50.059 104.351 102.8zM256 234.375a117.188 117.188 0 1 0-117.188-117.187A117.32 117.32 0 0 0 256 234.375zM256 32a85.188 85.188 0 1 1-85.188 85.188A85.284 85.284 0 0 1 256 32z"
        data-original="\#000000"></path>
    </svg>
    "#.to_string()
}

fn logout_icon() -> String {
    r#"
    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-4 h-4 mr-3"
        viewBox="0 0 6.35 6.35">
        <path
            d="M3.172.53a.265.266 0 0 0-.262.268v2.127a.265.266 0 0 0 .53 0V.798A.265.266 0 0 0 3.172.53zm1.544.532a.265.266 0 0 0-.026 0 .265.266 0 0 0-.147.47c.459.391.749.973.749 1.626 0 1.18-.944 2.131-2.116 2.131A2.12 2.12 0 0 1 1.06 3.16c0-.65.286-1.228.74-1.62a.265.266 0 1 0-.344-.404A2.667 2.667 0 0 0 .53 3.158a2.66 2.66 0 0 0 2.647 2.663 2.657 2.657 0 0 0 2.645-2.663c0-.812-.363-1.542-.936-2.03a.265.266 0 0 0-.17-.066z"
            data-original="\#000000"></path>
    </svg>
    "#.to_string()
}
