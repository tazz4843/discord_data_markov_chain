use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Record {
    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "Timestamp")]
    pub timestamp: String,

    #[serde(rename = "Contents")]
    pub contents: String,

    #[serde(rename = "Attachments")]
    pub attachments: Option<String>,
}
