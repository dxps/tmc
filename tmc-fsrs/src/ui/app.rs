use crate::ui::routes::Route;
use crate::ui::State;
use dioxus::prelude::*;

pub fn App() -> Element {
    //
    _ = console_log::init_with_level(log::Level::Debug);

    let state = State::load_from_localstorage();
    use_context_provider(|| state);

    rsx! {
        Router::<Route> {}
    }
}
