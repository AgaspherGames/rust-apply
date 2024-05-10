#![allow(non_snake_case)]

use crate::api::{get_internship, get_internships};
use crate::components::internshipCard::InternshipCard;
use dioxus::prelude::*;

#[component]
pub fn InternshipsPage() -> Element {
    let internshipResponse = use_resource(move || get_internships());

    match &*internshipResponse.read_unchecked() {
        Some(Ok(response)) => {
            rsx! {
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 ",
                    for internship in response.internships.iter() {
                        InternshipCard {
                            internship: internship.clone()
                        }
                    }
                }
            }
        }
        Some(Err(err)) => {
            // if there was an error, render the error
            rsx! {"An error occurred while fetching stories {err}"}
        }
        None => {
            // if the future is not resolved yet, render a loading message
            rsx! {"Loading items"}
        }
    }
}
