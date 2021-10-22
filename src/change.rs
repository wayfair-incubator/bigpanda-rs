use crate::{BigPandaError, Client, POST};
use hyper::Body;
use serde::Serialize;
use std::collections::HashMap;

const CHANGE_URL: &str = "https://api.bigpanda.io/data/changes";

/// Change Payload
/// See https://docs.bigpanda.io/reference#create-or-update-a-change for more details.
#[derive(Serialize, Debug)]
pub struct Change {
    /// The change's unique identifier from its original change system.
    pub identifier: String,

    /// Status of the change. Choose one: [Planned,In Progress,Done,Canceled].
    pub status: String,

    /// Short summary/title of the change.
    pub summary: String,

    /// Start time of the change in Unix Epochs.
    pub start: i64,

    /// End time of the change in Unix Epochs.
    pub end: i64,

    /// (Optional) The URL of the record in the change system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_url: Option<String>,

    /// (Optional) JSON object that keeps a record of all tags. Visible in the UI in the Related Changes tab..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,

    /// (Optional) JSON object that saves any data that's not in the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl Client {
    pub async fn send_change(&self, change: Change) -> Result<(), BigPandaError> {
        self.send_request(
            POST,
            CHANGE_URL,
            Body::from(serde_json::to_string(&change).unwrap()),
        )
        .await
    }
}