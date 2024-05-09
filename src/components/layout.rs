#![allow(non_snake_case)]

use crate::components::header::Header;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "w-full bg-stone-900",
            Header {}
        }
    }
}
