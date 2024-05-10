#![allow(non_snake_case)]

use crate::components::header::Header;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct LayoutProps {
    body: Element,
}

pub fn Layout(props: LayoutProps) -> Element {
    rsx! {
        div {
            class: "w-full min-h-screen bg-black text-white",
            Header {}
            {props.body}
        }
    }
}
