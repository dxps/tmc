use dioxus::prelude::*;

use crate::ui::comps::nav::common::{style_link, NavProps};
use crate::ui::routes::Route;
use crate::ui::State;

pub fn NavUserMenu(props: NavProps) -> Element {
    //
    let state = use_context::<Signal<State>>();

    if state().current_user.is_none() {
        log::debug!(">>> [NavUserMenu] There is no current user.");
        rsx! {
            Link {
                    class: style_link(&props.active_path, NavProps::login()).to_owned() + "hidden sm:inline-block sm:ml-auto sm:mr-3",
                    to: Route::Login {}, "Login",
            }
        }
    } else {
        log::debug!(">>> [NavUserMenu] There is a current user: {:?}.", state().current_user);
        rsx! {
            { "UserMenu" }
        }
    }
}
