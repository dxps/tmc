use crate::ui::routes::Route;
use crate::ui::State;
use dioxus::prelude::*;

pub fn App() -> Element {
    //
    _ = console_log::init_with_level(log::Level::Debug);

    _ = use_context_provider(|| Signal::new(State::default()));

    // Asynchronously loading state from localstorage and notify its value through the signal.
    use_future(move || async move {
        let mut state = use_context::<Signal<State>>();
        let local_state = State::load_from_localstorage();
        *state.write() = local_state();
        *APP_READY.write() = true;
    });

    rsx! {
        Router::<Route> {}
    }
}

pub static APP_READY: GlobalSignal<bool> = GlobalSignal::new(|| false);
