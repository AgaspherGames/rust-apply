use crate::config::BACKEND_URL;
use serde::{Deserialize, Serialize};
use tracing::info;

// pub static BASE_API_URL = BACKEND_URL;

#[derive(Deserialize, PartialEq, Clone, Debug, Default)]
pub(crate) struct Internship {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) company: Company,
    pub(crate) city: City,
    pub(crate) is_paid: bool,
    pub(crate) is_active: bool,
    pub(crate) max_interns: u32,
    pub(crate) attachment: String,
    pub(crate) keywords: Vec<String>,
    pub(crate) created_at: String,
}
#[derive(Deserialize, PartialEq, Clone, Debug, Default)]
pub(crate) struct Company {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) avatar: String,
}
#[derive(Deserialize, PartialEq, Clone, Debug, Default)]
pub(crate) struct City {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) area_id: u32,
}
#[derive(Deserialize, PartialEq, Clone, Debug, Default)]
pub(crate) struct InternshipsResponse {
    pub(crate) cursor: String,
    pub(crate) internships: Vec<Internship>,
}

pub async fn get_internships() -> Result<InternshipsResponse, reqwest::Error> {
    // for (key, value) in env::vars() {
    //     info!(key);
    // }

    info!("-------");
    let url = format!("{}/{}", BACKEND_URL, "internship");
    reqwest::get(&url).await?.json().await
}
pub async fn get_internship() -> Result<Internship, reqwest::Error> {
    let url = format!(
        "{}/{}",
        BACKEND_URL, "internship/3e7d9532-207f-4241-b420-59be0bb4877b"
    );
    reqwest::get(&url).await?.json().await
}
