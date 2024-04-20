#![allow(non_snake_case)]

#[cfg(feature = "server")]
mod auth;
mod server;

mod ui;

use dioxus::prelude::*;

use crate::ui::Route;

fn main() {
    //
    //launch(App);
    #[cfg(feature = "web")]
    // Hydrate the application on the client.
    dioxus_web::launch::launch_cfg(ui::App, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    server::server_start(ui::App)
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to Blog"
        }
        div { class: "",
            h1 { "High-Five counter: {count}" }
            button { class: "bg-slate-300 rounded-lg m-2 px-2 py-1", onclick: move |_| count += 1, "Up high!" }
            button { class: "bg-slate-400 rounded-lg m-2 px-2 py-1", onclick: move |_| count -= 1, "Down low!" }
            button { class: "bg-slate-100 rounded-lg m-2 px-2 py-1",
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        println!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
