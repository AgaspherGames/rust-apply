#![allow(non_snake_case)]

use crate::api::get_internship;
use crate::components::{
    card::Card, is_paid_card::IsPaidCard, keywords_list::KeywordsList, layout::Layout,
    loading_screen::LoadingScreen,
};
use crate::config::FILES_URL;
use chrono::DateTime;

use dioxus::prelude::*;

#[component]
pub fn InternshipPage(id: String) -> Element {
    let response = use_resource(move || get_internship(id.clone()));

    rsx! {
        Layout{
            body: match &*response.read_unchecked() {
                Some(Ok(internship)) => {
                    let parsed_date = DateTime::parse_from_rfc3339(internship.created_at.as_str()).expect("Failed to parse datetime");
                    let formatted_date = parsed_date.format("%d.%m.%Y").to_string();
                    rsx! {
                        div {
                            class: "max-w-screen-md mx-auto mt-12",
                            Card {
                                class: "w-fit ",
                                body: rsx!{

                                    div{
                                        class: "flex items-center my-1 gap-2 ",
                                        img{
                                            src: "{FILES_URL}{internship.company.avatar}",
                                            class: "aspect-square object-cover w-8 h-8 rounded-full ",
                                            alt: "company image"
                                        }
                                        p {
                                            class: "text-xl",
                                            "{internship.company.name}"
                                        }
                                    }
                                }
                            }
                            Card{
                                class: "group relative mt-4 ",
                                body: rsx!{
                                    div {
                                        class: " flex justify-between",
                                        p {
                                            class: "w-fit flex items-center text-lg text-green-500",
                                            "Активно"
                                        }
                                        div {
                                            class: "flex gap-2",
                                            p {
                                                class:"w-fit flex items-center text-zinc-500 ",
                                                "{internship.city.name}"
                                            }
                                            p {
                                                class:"w-fit flex items-center text-zinc-500 ",
                                                "{formatted_date}"
                                            }

                                        }
                                    }
                                    h2 {
                                        class: "text-3xl",
                                        "{internship.name}"
                                    }
                                    div {
                                        class: "flex flex-col gap-4 my-4 ",
                                        IsPaidCard{
                                            is_paid: internship.is_paid
                                        }
                                    }
                                    p {
                                        class: "whitespace-pre-wrap",
                                        "{internship.description}"
                                    }
                                    KeywordsList {
                                        keywords: internship.keywords.clone()
                                    }

                                }
                            }
                        }
                    }
                }
                Some(Err(err)) => {
                    rsx! {"An error occurred while fetching stories {err}"}
                }
                None => {
                    rsx! {
                        LoadingScreen {}
                    }
                }
            }
        }
    }
}
