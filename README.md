# bigpanda-rs

`bigpanda-rs` is a Rust library for integrating with the [BigPanda API](https://docs.bigpanda.io/reference).

The library currently supports creating/updating alerts and changes.

## Usage Guide

Please see the tests in this respository for thorough examples, but here's a quick guide.

First, create a BigPanda client with a specific Api type along with an app key and auth_token:

```rust
use bigpanda_rs::Client;
use bigpanda_rs::alert::{Alert, AlertStatus};
use bigpanda_rs::change::Change;

let client = Client::new(
    ApiType::Alert,
    &app_key.to_string(),
    &auth_token.to_string()
);
```

Second, build your alert or change payload (See the tests for some more details):

### Alert Payload
```rust
let alert = Alert {
    app_key: app_key.to_string(),
    status: AlertStatus::Ok,
    host: "host_name".to_string(),
    timestamp: Some(timestamp),
    description: Some("This is a description".to_string()),
    check: Some("This is a check".to_string()),
    cluster: Some("cluster_name".to_string()),
    primary_property: Some("host".to_string()),
    secondary_property: None,
    additional: None,
};
```

### Change Payload
```rust
let now = Utc::now();
let timestamp: i64 = now.timestamp();

let mut tags = HashMap::new();
tags.insert("change_type".to_string(), "software".to_string());
tags.insert("service".to_string(), "test_service".to_string());

let change = Change {
    identifier: "TEST-12345".to_string(),
    status: "Done".to_string(),
    summary: "This is a test change".to_string(),
    start: timestamp,
    end: timestamp,
    tags: Some(tags),
    ticket_url: None,
    metadata: None,
};
```

Finally, send the respective request:

### Send an Alert
```rust
let response = client.send_alert(alert).await;
```

### Send a Change
```rust
let response = client.send_change(change).await;
```
