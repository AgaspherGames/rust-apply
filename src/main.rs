#![allow(non_snake_case)]
extern crate chrono;

use crate::components::layout::Layout;
use crate::pages::{internship_page::InternshipPage, internships_page::InternshipsPage};
use components::layout;
use dioxus::prelude::*;
use dotenv::dotenv;
use std::env;
use tracing::{info, Level};

mod api;
pub(crate) mod components;
mod config;
pub(crate) mod pages;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/internship/:id")]
    InternshipPage { id: String },
    #[end_layout]
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    dotenv().ok();
    // let backend_url = env::var("BACKEND_URL").unwrap();
    println!("{:?}", env::args());

    // let files_url = env::var("FILES_URL").expect("FILES_URL must be set");

    // println!("Database URL: {}", backend_url);
    // println!("API Key: {}", files_url);

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    info!("aaaa");

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Layout{
            body: rsx! {
                div {
                    class: "p-4 ",
                    InternshipsPage {}
                }
            }
        }
    }
}
