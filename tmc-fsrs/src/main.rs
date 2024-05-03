#![allow(non_snake_case)]

#[cfg(feature = "web")]
use dioxus::launch;

#[cfg(feature = "server")]
mod auth;
mod server;

mod ui;

fn main() {
    #[cfg(feature = "web")]
    // Hydrate the application on the client.
    // dioxus_web::launch::launch_cfg(ui::App, dioxus_web::Config::new().hydrate(true));
    //
    launch(ui::App);

    #[cfg(feature = "server")]
    server::start(ui::App)
}
