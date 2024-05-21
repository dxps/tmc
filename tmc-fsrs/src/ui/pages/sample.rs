use dioxus::prelude::*;

use crate::{
    server::fns::sample::{get_server_data, post_server_data},
    ui::{
        comps::{Nav, NavProps},
        routes::Route,
    },
};

#[component]
pub fn Sample() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div {
            class: "flex flex-col min-h-screen bg-gray-100",
            Nav { active_path: NavProps::sample() },
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3",
                    div {
                        h1 { class:"text-center text-4xl text-bold", "{count}" }
                        button { class: "bg-slate-300 rounded-lg m-2 px-2 py-1", onclick: move |_| count += 1, "Up high!" }
                        button { class: "bg-slate-400 rounded-lg m-2 px-2 py-1", onclick: move |_| count -= 1, "Down low!" }
                        Link { class: "pl-2", to: Route::Blog { id: count() }, "Go to Blog {count}" }
                    }
                    div {
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
                div { class: "pt-8",
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}
