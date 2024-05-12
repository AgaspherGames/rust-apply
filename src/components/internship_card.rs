#![allow(non_snake_case)]

use crate::config::FILES_URL;
use crate::{
    api::Internship,
    components::{card::Card, is_paid_card::IsPaidCard, keywords_list::KeywordsList},
};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InternshipCardProps {
    internship: Internship,
}

#[component]
pub fn InternshipCard(props: InternshipCardProps) -> Element {
    let url = format!("/internship/{}", props.internship.id);
    rsx! {
        Link {
            to: url,
            class: "block",
            Card {
                class: " h-full flex flex-col ",
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
                        IsPaidCard{
                            is_paid: props.internship.is_paid
                        }
                    }

                    p {
                        class: "mt-3 line-clamp-3",
                        "{props.internship.description}"
                    }
                    div {
                        class: "mt-auto ",
                        KeywordsList{
                            keywords: props.internship.keywords.clone()
                        }
                    }
                }
            }
        }
    }
}
