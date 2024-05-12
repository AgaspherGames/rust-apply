#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn LoadingScreen() -> Element {
    rsx! {
        div {
            class: "fixed top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2",

            div {
                class: "border-blue-500 inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]",

                span {
                    class: "!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]",
                    "Loading..."
                }
            }
        }
    }
}
