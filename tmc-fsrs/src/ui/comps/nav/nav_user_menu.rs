use dioxus::prelude::*;

use crate::ui::comps::{style_nav_item_link, style_nav_item_user_menu, NavProps};
use crate::ui::routes::Route;
use crate::ui::{State, APP_READY};

pub fn NavUserMenu(props: NavProps) -> Element {
    //
    let state = use_context::<Signal<State>>();
    let mut show_dropdown = use_signal(|| false);

    if *APP_READY.read() == false {
        return rsx! {};
    };

    if state().current_user.is_none() {
        log::debug!(">>> [NavUserMenu] There is no locally saved user.");
        rsx! {
            Link {
                class: style_nav_item_link(&props.active_path, NavProps::login()).to_owned()
                    + "sm:inline-block sm:ml-auto sm:mr-3",
                to: Route::Login {},
                "Login"
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
                class: style_nav_item_user_menu(&props.active_path).to_owned()
                    + " flex flex-col items-end overflow-visible",
                button {
                    class: "px-8 align  rounded-lg text-sm outline-none",
                    onclick: move |_| {
                        let curr_val = show_dropdown();
                        *show_dropdown.write() = !curr_val;
                    },
                    div {
                        class: "rounded-full",
                        dangerous_inner_html: btn_user_icon()
                    }
                }
                if show_dropdown() {
                    NavUserDropdown { username, show_dropdown }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct NavUserDropdownProps {
    username: String,
    show_dropdown: Signal<bool>,
}

fn NavUserDropdown(mut props: NavUserDropdownProps) -> Element {
    //
    rsx! {
        div {
            // "style": "width: 100%; height: 1000%; padding: 0; position: absolute; top: 0; left: 0; z-index: 40",
            "style": "width: 100%; height: 1000%; padding: 0; position: absolute; top: 0; left: 0",
            onclick: move |_| {
                log::debug!(">>> [NavUserDropdown] Clicked in the outer div!");
                *props.show_dropdown.write() = false;
            },
            div { class: "w-20 mt-14 mr-28 bg-white rounded-lg shadow-2xl float-right",
                div {
                    ul { class: "shadow-lg bg-white py-2 z-[1000] min-w-full w-max rounded-lg max-h-96 overflow-auto",
                        li { class: "py-2.5 px-12 flex items-center text-[#888] text-sm",
                            "{props.username} user menu"
                        }
                        li { class: "flex items-center text-[#333] hover:bg-gray-100 hover:text-orange-600 text-sm cursor-pointer",
                            Link {
                                class: "py-2.5 px-5 min-w-full w-max min-h-full flex text-[#333]",
                                to: Route::UserProfile {
                                    username: props.username,
                                },
                                div { dangerous_inner_html: user_icon() }
                                "My profile"
                            }
                        }
                        li { class: "flex items-center text-[#333] hover:bg-gray-100 hover:text-orange-600 text-sm cursor-pointer",
                            Link {
                                class: "py-2.5 px-5 min-w-full w-max min-h-full flex text-[#333]",
                                to: Route::Logout {},
                                div { dangerous_inner_html: logout_icon() }
                                "Logout"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn btn_user_icon() -> String {
    r#"
    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-5 h-5" viewBox="0 0 512 512">
        <path
            d="M337.711 241.3a16 16 0 0 0-11.461 3.988c-18.739 16.561-43.688 25.682-70.25 
               25.682s-51.511-9.121-70.25-25.683a16.007 16.007 0 0 0-11.461-3.988c-78.926 4.274-140.752 63.672-140.752 
               135.224v107.152C33.537 499.293 46.9 512 63.332 512h385.336c16.429 0 29.8-12.707 
               29.8-28.325V376.523c-.005-71.552-61.831-130.95-140.757-135.223zM446.463 480H65.537V376.523c0-52.739 
               45.359-96.888 104.351-102.8C193.75 292.63 224.055 302.97 256 302.97s62.25-10.34 86.112-29.245c58.992 5.91 
               104.351 50.059 104.351 102.8zM256 234.375a117.188 117.188 0 1 0-117.188-117.187A117.32 117.32 0 0 0 256 
               234.375zM256 32a85.188 85.188 0 1 1-85.188 85.188A85.284 85.284 0 0 1 256 32z"
            data-original="\#000000"></path>
    </svg>
    "#
    .to_string()
}

fn user_icon() -> String {
    r#"
    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-4 h-4 mr-3" viewBox="0 0 512 512">
        <path
            d="M337.711 241.3a16 16 0 0 0-11.461 3.988c-18.739 16.561-43.688 25.682-70.25 
               25.682s-51.511-9.121-70.25-25.683a16.007 16.007 0 0 0-11.461-3.988c-78.926 4.274-140.752 
               63.672-140.752 135.224v107.152C33.537 499.293 46.9 512 63.332 512h385.336c16.429 0 29.8-12.707 
               29.8-28.325V376.523c-.005-71.552-61.831-130.95-140.757-135.223zM446.463 480H65.537V376.523c0-52.739 
               45.359-96.888 104.351-102.8C193.75 292.63 224.055 302.97 256 302.97s62.25-10.34 86.112-29.245c58.992 5.91 
               104.351 50.059 104.351 102.8zM256 234.375a117.188 117.188 0 1 0-117.188-117.187A117.32 
               117.32 0 0 0 256 234.375zM256 32a85.188 85.188 0 1 1-85.188 85.188A85.284 85.284 0 0 1 256 32z"
            data-original="\#000000"></path>
    </svg>
    "#
    .to_string()
}

fn logout_icon() -> String {
    r#"
    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-4 h-4 mr-3"
        viewBox="0 0 6.35 6.35">
        <path
            d="M3.172.53a.265.266 0 0 0-.262.268v2.127a.265.266 0 0 0 .53 0V.798A.265.266 0 0 0 3.172.53zm1.544.532a.265.266 
               0 0 0-.026 0 .265.266 0 0 0-.147.47c.459.391.749.973.749 1.626 0 1.18-.944 2.131-2.116 2.131A2.12 2.12 0 0 1 
               1.06 3.16c0-.65.286-1.228.74-1.62a.265.266 0 1 0-.344-.404A2.667 2.667 0 0 0 .53 3.158a2.66 2.66 0 0 0 2.647 
               2.663 2.657 2.657 0 0 0 2.645-2.663c0-.812-.363-1.542-.936-2.03a.265.266 0 0 0-.17-.066z"
            data-original="\#000000"></path>
    </svg>
    "#
    .to_string()
}
