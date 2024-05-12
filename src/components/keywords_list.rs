use dioxus::prelude::*;
use crate::components::card::Card;

#[derive(PartialEq, Clone, Props)]
pub struct KeywordsListProps {
    keywords: Vec<String>,
}

pub fn KeywordsList(props: KeywordsListProps) -> Element {
    rsx! {
        div{
            class: "w-full overflow-scroll",
            div {
                class: "flex gap-2 mt-2 w-max ",
                for word in props.keywords{
                    Card {
                        class: "w-fit py-1.5 px-3 ",
                        body: rsx! {
                            p {
                                "{word}"
                            }
                        }
                    }
                }
            }
        }
    }
}
