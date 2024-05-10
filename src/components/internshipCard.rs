#![allow(non_snake_case)]

use crate::{api::Internship, components::card::Card};
use dioxus::prelude::*;
use crate::config::FILES_URL;

#[derive(PartialEq, Clone, Props)]
pub struct InternshipCardProps {
    internship: Internship,
}
#[derive(PartialEq, Clone, Props)]
pub struct KeywordsListProps {
    keywords: Vec<String>,
}

fn PaymentCard() -> Element {
    rsx! {
        Card {
            class: "text-green-500 flex gap-2 items-center py-2 px-4 w-min ",
            body: rsx! {
                p{
                    "Оплачиваемая"
                }
            }
        }
    }
}

fn KeywordsList(props: KeywordsListProps) -> Element {
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

pub fn InternshipCard(props: InternshipCardProps) -> Element {
    rsx! {
        Card {
            body: rsx! {
                div {
                    class: " flex justify-between",
                    div {
                        class: "flex items-center gap-2",
                        p{
                            class: "text-2xl",
                            "{props.internship.name}"
                        }
                    }
                }

                div {
                    class: "flex flex-wrap justify-between",
                    div{
                        class: "flex items-center my-1 gap-1 ",
                        img{
                            src: "{FILES_URL}{props.internship.company.avatar}",
                            class: "aspect-square object-cover w-6 h-6 rounded-full ",
                            alt: "company image"
                        }
                        p {
                            "{props.internship.company.name}"
                        }
                    }
                    div{
                        class: "w-fit flex items-center text-zinc-500",
                        p {
                            class: "line-clamp-1",
                            "{props.internship.city.name}"
                        }
                    }
                }

                div {
                    class: "flex flex-col gap-2 my-2 md:flex-row",
                    PaymentCard{}
                }

                p {
                    class: "mt-3 line-clamp-3",
                    "{props.internship.description}"
                }

                KeywordsList{
                    keywords: props.internship.keywords.clone()
                }
            }
        }
    }
}
