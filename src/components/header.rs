#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "w-full bg-stone-900 text-stone-300 flex justify-between py-4 px-8",
            div{
                Link {
                    "Apply",
                    to: "/"
                }
            }

        }
    }
}
