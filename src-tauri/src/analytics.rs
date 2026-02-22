use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event: String,
    pub properties: Option<serde_json::Value>,
}

// ===== RELEASE BUILD: Full analytics implementation =====
#[cfg(not(debug_assertions))]
mod release {
    use serde_json::json;

    const POSTHOG_API_KEY: &str = env!("POSTHOG_API_KEY");
    const POSTHOG_HOST: &str = "https://us.i.posthog.com";

    /// Track an analytics event to PostHog
    /// Fails silently if analytics is disabled or if the request fails
    pub async fn track_event(
        event_name: String,
        properties: Option<serde_json::Value>,
        analytics_enabled: bool,
        installation_id: String,
        session_id: String,
    ) -> Result<(), String> {
        // Respect opt-out
        if !analytics_enabled {
            return Ok(());
        }

        // Use installation_id as the distinct_id for stable user tracking
        let distinct_id = installation_id;

        let mut props = properties.unwrap_or_else(|| json!({}));

        // Add platform, app version, and session tracking
        if let Some(obj) = props.as_object_mut() {
            obj.insert("platform".to_string(), json!(std::env::consts::OS));
            obj.insert("app_version".to_string(), json!(env!("CARGO_PKG_VERSION")));
            // PostHog uses $session_id for session tracking
            obj.insert("$session_id".to_string(), json!(session_id));
        }

        let payload = json!({
            "api_key": POSTHOG_API_KEY,
            "event": event_name,
            "properties": props,
            "distinct_id": distinct_id,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            // Set person properties for better user identification
            "$set": {
                "platform": std::env::consts::OS,
                "app_version": env!("CARGO_PKG_VERSION"),
            }
        });

        // Send async, fail silently
        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let url = format!("{}/capture/", POSTHOG_HOST);

            let _ = client.post(&url).json(&payload).send().await;
        });

        Ok(())
    }
}

// ===== DEBUG BUILD: No-op stub implementation =====
#[cfg(debug_assertions)]
mod debug {
    /// Debug stub: analytics disabled in development builds
    pub async fn track_event(
        _event_name: String,
        _properties: Option<serde_json::Value>,
        _analytics_enabled: bool,
        _installation_id: String,
        _session_id: String,
    ) -> Result<(), String> {
        // No-op in debug builds
        Ok(())
    }
}

// Export the appropriate implementation based on build type
#[cfg(not(debug_assertions))]
pub use release::track_event;

#[cfg(debug_assertions)]
pub use debug::track_event;
