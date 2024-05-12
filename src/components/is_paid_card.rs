use crate::{api::Internship, components::card::Card};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct IsPaidCardProps {
    is_paid: bool,
}

pub fn IsPaidCard(props: IsPaidCardProps) -> Element {
    if props.is_paid {
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
    } else {
        rsx! {
            Card {
                class: "text-zinc-500 flex gap-2 items-center py-2 px-4 w-min ",
                body: rsx! {
                    p{
                        "Неоплачиваемая"
                    }
                }
            }
        }
    }
}
