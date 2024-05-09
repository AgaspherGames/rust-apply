#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        div {
            class: "w-full bg-stone-900 text-stone-300 flex justify-between py-4 px-8",
            div{
                p {
                    "Apply"
                }
            }
            div{
                button {
                    "LogIn",
                    // onclick: move |_| count += 1
                }
            }
        }
    }
}
