mod asynctest {
    use bigpanda_rs::alert::{Alert, AlertStatus};
    use bigpanda_rs::change::Change;
    use bigpanda_rs::{ApiType, Client};
    use chrono::Utc;
    use std::collections::HashMap;
    use std::env;

    #[tokio::test]
    async fn send_alert_critical() {
        let app_key = env::var("BP_APP_KEY").expect("Find BP_APP_KEY environment variable");
        let auth_token = env::var("BP_AUTH_TOKEN").expect("Find BP_AUTH_TOKEN environment variable");

        let now = Utc::now();
        let timestamp: i64 = now.timestamp();

        let mut map = HashMap::new();
        map.insert("test_1".to_string(), "test_1".to_string());
        map.insert("test_2".to_string(), "test_2".to_string());

        let alert = Alert {
            app_key: app_key.to_string(),
            status: AlertStatus::Critical,
            host: "rust_test_host".to_owned(),
            timestamp: Some(timestamp),
            description: Some("rust test description".to_string()),
            check: None,
            cluster: None,
            primary_property: None,
            secondary_property: None,
            additional: Some(map),
        };

        let client = Client::new(
            ApiType::Alert,
            &app_key.to_string(),
            &auth_token.to_string(),
        );

        let response = client.send_alert(alert).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn send_alert_ok() {
        let app_key = env::var("BP_APP_KEY").expect("Find BP_APP_KEY environment variable");
        let auth_token = env::var("BP_AUTH_TOKEN").expect("Find BP_AUTH_TOKEN environment variable");

        let now = Utc::now();
        let timestamp: i64 = now.timestamp();

        let mut map = HashMap::new();
        map.insert("test_1".to_string(), "test_1".to_string());
        map.insert("test_2".to_string(), "test_2".to_string());

        let alert = Alert {
            app_key: app_key.to_string(),
            status: AlertStatus::Ok,
            host: "rust_test_host".to_string(),
            timestamp: Some(timestamp),
            description: Some("rust test description".to_string()),
            check: None,
            cluster: None,
            primary_property: None,
            secondary_property: None,
            additional: Some(map),
        };

        let client = Client::new(
            ApiType::Alert,
            &app_key.to_string(),
            &auth_token.to_string()
        );

        let response = client.send_alert(alert).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn send_change() {
        let app_key = env::var("BP_CHANGE_APP_KEY").expect("Find BP_CHANGE_APP_KEY environment variable");
        let auth_token = env::var("BP_CHANGE_API_KEY").expect("Find BP_CHANGE_API_KEY environment variable");

        let now = Utc::now();
        let timestamp: i64 = now.timestamp();

        let mut tags = HashMap::new();
        tags.insert("change_type".to_string(), "software".to_string());
        tags.insert("service".to_string(), "test_service".to_string());

        let change = Change {
            identifier: "TEST-12345".to_string(),
            status: "Done".to_string(),
            summary: "This is a test change".to_string(),
            tags: Some(tags),
            start: timestamp,
            end: timestamp,
            ticket_url: None,
            metadata: None,
        };

        let client = Client::new(
            ApiType::Change,
            &app_key.to_string(),
            &auth_token.to_string()
        );

        let response = client.send_change(change).await;

        assert!(response.is_ok());
    }
}
