#![allow(non_snake_case)]

#[cfg(feature = "web")]
use dioxus::launch;

#[cfg(feature = "server")]
mod auth;
mod server;

mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    #[cfg(feature = "server")]
    dotenvy::dotenv()?;

    #[cfg(feature = "server")]
    server::start(ui::App);

    #[cfg(feature = "web")]
    launch(ui::App);

    Ok(())
}
