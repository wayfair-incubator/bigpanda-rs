use crate::{BigPandaError, Client, POST};
use hyper::Body;
use serde::Serialize;
use std::collections::HashMap;

const ALERT_URL: &str = "https://api.bigpanda.io/data/v2/alerts";

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    Acknowledged,
    Critical,
    Ok,
    Warning,
}

/// Alert Payload
/// See https://docs.bigpanda.io/reference#alerts for more details.
#[derive(Serialize, Debug)]
pub struct Alert {
    /// Application key.
    pub app_key: String,

    /// Status of the alert. One of [ ok, critical, warning, acknowledged ].
    pub status: AlertStatus,

    /// Main object that caused the alert. Can be the associated host or,
    /// if a host isn't relevant, a service or an application. If you want to include
    /// more than one of these fields, consider specifying the primary and secondary properties.
    pub host: String,

    /// (Optional) Time that the alert occurred in Unix format (UTC timezone).
    /// If the time is not specified, the value defaults to the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,

    /// (Optional) Secondary object or sub-item that caused the alert (often shown as an incident subtitle in BigPanda).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check: Option<String>,

    /// (Optional) Brief summary (max. 2048 characters) of the alert for certain monitoring tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// (Optional) Server cluster or logical host-group from which the alert was sent.
    /// This value is used to correlate alerts into high-level incidents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,

    /// (Optional) BigPanda uses the primary property to construct the title of an incident.
    /// By default, the primary property is defined as one of the following fields: host, service, application, or device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_property: Option<String>,

    /// (Optional) BigPanda uses the secondary property to construct the subtitle of an incident.
    /// By default, the secondary property is defined as one of the following fields: check or sensor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_property: Option<String>,

    ///(Optional) Additional information you want to have available in BigPanda. 
    /// You can add any number of custom JSON attributes with a string value to the payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub additional: Option<HashMap<String, String>>,
}

impl Client {
    pub async fn send_alert(&self, alert: Alert) -> Result<(), BigPandaError> {
        self.send_request(
            POST,
            ALERT_URL,
            Body::from(serde_json::to_string(&alert).unwrap()),
        )
        .await
    }
}
