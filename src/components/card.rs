#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    #[props(into)]
    class: Option<String>,
    body: Element,
}

pub fn Card(props: CardProps) -> Element {
    let mut className = "p-4 rounded-lg  bg-stone-800 bg-opacity-40 overflow-hidden ".to_string();
    if (props.class.is_some()){
        className = props.class.unwrap_or_else(|| "".to_string())+ className.as_mut_str();
    }
    rsx! {
        div {
            class: className,
            {props.body}
        }
    }
}
