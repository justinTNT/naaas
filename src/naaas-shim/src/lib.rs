use anyhow::Result;
use hyper::{Body, Client, Method, Request, Response, StatusCode, Uri};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub name: String,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
}

#[derive(Clone)]
pub struct ProxyState {
    pub upstream_url: String,
    pub client: Client<hyper::client::HttpConnector>,
    pub app_config: Option<AppConfig>,
}

impl ProxyState {
    pub fn new(upstream_url: String, app_config: Option<AppConfig>) -> Self {
        Self {
            upstream_url,
            client: Client::new(),
            app_config,
        }
    }
}

pub async fn handle_config(state: &ProxyState) -> Result<Response<Body>, anyhow::Error> {
    match &state.app_config {
        Some(config) => {
            let response = serde_json::to_string(config)?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(response))?)
        }
        None => {
            let default_config = AppConfig {
                name: "Default App".to_string(),
                logo_url: None,
                primary_color: Some("#007acc".to_string()),
            };
            let response = serde_json::to_string(&default_config)?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(response))?)
        }
    }
}

pub async fn proxy_request(
    mut req: Request<Body>,
    state: &ProxyState,
) -> Result<Response<Body>, anyhow::Error> {
    info!("Proxying {} {}", req.method(), req.uri().path());
    
    // Build the upstream URL
    let upstream_uri = format!(
        "{}{}{}",
        state.upstream_url.trim_end_matches('/'),
        req.uri().path(),
        req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default()
    );
    
    let uri: Uri = upstream_uri.parse()?;
    *req.uri_mut() = uri;
    
    // Remove hop-by-hop headers
    req.headers_mut().remove("host");
    req.headers_mut().remove("connection");
    req.headers_mut().remove("upgrade");
    req.headers_mut().remove("proxy-authorization");
    req.headers_mut().remove("proxy-authenticate");
    req.headers_mut().remove("te");
    req.headers_mut().remove("trailer");
    req.headers_mut().remove("transfer-encoding");
    
    // Forward the request to the upstream
    let response = state.client.request(req).await?;
    Ok(response)
}

pub async fn router(
    req: Request<Body>,
    state: &ProxyState,
) -> Result<Response<Body>, Infallible> {
    let result = match (req.method(), req.uri().path()) {
        (&Method::GET, "/config") => {
            handle_config(state).await
        }
        _ => {
            proxy_request(req, state).await
        }
    };
    
    match result {
        Ok(resp) => Ok(resp),
        Err(err) => {
            error!("Request handler error: {}", err);
            Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from("Bad Gateway"))
                .unwrap())
        }
    }
}

pub fn build_upstream_uri(upstream_url: &str, path: &str, query: Option<&str>) -> String {
    format!(
        "{}{}{}",
        upstream_url.trim_end_matches('/'),
        path,
        query.map(|q| format!("?{}", q)).unwrap_or_default()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{Body, Method, Request, StatusCode};

    #[test]
    fn test_app_config_serialization_with_all_fields() {
        // Setup: Create app config with all fields
        let config = AppConfig {
            name: "Test App".to_string(),
            logo_url: Some("https://example.com/logo.png".to_string()),
            primary_color: Some("#ff0000".to_string()),
        };

        // Action: Serialize to JSON
        let json = serde_json::to_string(&config).unwrap();

        // Assert: JSON contains all fields
        assert!(json.contains("Test App"));
        assert!(json.contains("https://example.com/logo.png"));
        assert!(json.contains("#ff0000"));
    }

    #[test] 
    fn test_app_config_serialization_with_minimal_fields() {
        // Setup: Create app config with only required fields
        let config = AppConfig {
            name: "Minimal App".to_string(),
            logo_url: None,
            primary_color: None,
        };

        // Action: Serialize to JSON
        let json = serde_json::to_string(&config).unwrap();

        // Assert: JSON contains name and null values
        assert!(json.contains("Minimal App"));
        assert!(json.contains("\"logo_url\":null"));
        assert!(json.contains("\"primary_color\":null"));
    }

    #[test]
    fn test_proxy_state_creation_stores_all_fields() {
        // Setup: Create proxy state with config
        let config = AppConfig {
            name: "Test Service".to_string(),
            logo_url: None,
            primary_color: Some("#blue".to_string()),
        };
        let state = ProxyState::new("http://localhost:3000".to_string(), Some(config.clone()));

        // Assert: All fields are stored correctly
        assert_eq!(state.upstream_url, "http://localhost:3000");
        assert_eq!(state.app_config, Some(config));
        // Note: Client is created and available for use
    }

    #[test]
    fn test_proxy_state_creation_without_config() {
        // Setup: Create proxy state without config
        let state = ProxyState::new("http://backend:8080".to_string(), None);

        // Assert: Upstream URL set, no config
        assert_eq!(state.upstream_url, "http://backend:8080");
        assert_eq!(state.app_config, None);
    }

    #[tokio::test]
    async fn test_handle_config_returns_configured_json() {
        // Setup: Create state with specific config
        let config = AppConfig {
            name: "Production App".to_string(),
            logo_url: Some("https://cdn.example.com/logo.svg".to_string()),
            primary_color: Some("#28a745".to_string()),
        };
        let state = ProxyState::new("http://upstream".to_string(), Some(config));

        // Action: Handle config request
        let response = handle_config(&state).await.unwrap();

        // Assert: Response has correct status and content-type
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap().to_str().unwrap(),
            "application/json"
        );

        // Assert: Body contains the configured values
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert!(body_str.contains("Production App"));
        assert!(body_str.contains("https://cdn.example.com/logo.svg"));
        assert!(body_str.contains("#28a745"));
    }

    #[tokio::test] 
    async fn test_handle_config_returns_default_when_no_config() {
        // Setup: Create state without config
        let state = ProxyState::new("http://upstream".to_string(), None);

        // Action: Handle config request
        let response = handle_config(&state).await.unwrap();

        // Assert: Response has correct status
        assert_eq!(response.status(), StatusCode::OK);

        // Assert: Body contains default configuration
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert!(body_str.contains("Default App"));
        assert!(body_str.contains("#007acc"));
    }

    #[tokio::test]
    async fn test_router_handles_config_endpoint() {
        // Setup: Create state and config request
        let state = ProxyState::new("http://upstream".to_string(), None);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/config")
            .body(Body::empty())
            .unwrap();

        // Action: Route the request
        let response = router(request, &state).await.unwrap();

        // Assert: Config endpoint returns OK status
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap().to_str().unwrap(),
            "application/json"
        );
    }

    #[test]
    fn test_build_upstream_uri_with_path_only() {
        // Setup: Basic upstream URL and path
        let upstream = "http://localhost:8080";
        let path = "/api/users";

        // Action: Build URI
        let result = build_upstream_uri(upstream, path, None);

        // Assert: URI is constructed correctly
        assert_eq!(result, "http://localhost:8080/api/users");
    }

    #[test]
    fn test_build_upstream_uri_with_trailing_slash_removal() {
        // Setup: Upstream URL with trailing slash
        let upstream = "http://localhost:8080/";
        let path = "/health";

        // Action: Build URI
        let result = build_upstream_uri(upstream, path, None);

        // Assert: No double slashes in result
        assert_eq!(result, "http://localhost:8080/health");
    }

    #[test]
    fn test_build_upstream_uri_with_query_parameters() {
        // Setup: Path with query parameters
        let upstream = "http://api.example.com";
        let path = "/search";
        let query = Some("q=rust&limit=10");

        // Action: Build URI
        let result = build_upstream_uri(upstream, path, query);

        // Assert: Query parameters are included
        assert_eq!(result, "http://api.example.com/search?q=rust&limit=10");
    }

    #[test]
    fn test_build_upstream_uri_with_empty_query() {
        // Setup: Path without query parameters
        let upstream = "https://backend.service.local:9000";
        let path = "/status";

        // Action: Build URI
        let result = build_upstream_uri(upstream, path, None);

        // Assert: No query string appended
        assert_eq!(result, "https://backend.service.local:9000/status");
    }

    #[test]
    fn test_build_upstream_uri_with_root_path() {
        // Setup: Root path
        let upstream = "http://cms.internal";
        let path = "/";

        // Action: Build URI  
        let result = build_upstream_uri(upstream, path, None);

        // Assert: Root path handled correctly
        assert_eq!(result, "http://cms.internal/");
    }

    #[test]
    fn test_build_upstream_uri_handles_complex_query_string() {
        // Setup: Complex query with multiple parameters
        let upstream = "http://localhost:3000";
        let path = "/admin/posts";
        let query = Some("filter=published&sort=created_at&order=desc&page=2");

        // Action: Build URI
        let result = build_upstream_uri(upstream, path, query);

        // Assert: Complex query preserved
        assert_eq!(
            result, 
            "http://localhost:3000/admin/posts?filter=published&sort=created_at&order=desc&page=2"
        );
    }

    #[test]
    fn test_app_config_equality_for_same_values() {
        // Setup: Create two identical configs
        let config1 = AppConfig {
            name: "Same App".to_string(),
            logo_url: Some("logo.png".to_string()),
            primary_color: Some("#blue".to_string()),
        };
        let config2 = AppConfig {
            name: "Same App".to_string(), 
            logo_url: Some("logo.png".to_string()),
            primary_color: Some("#blue".to_string()),
        };

        // Assert: Configs are equal
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_app_config_inequality_for_different_names() {
        // Setup: Create configs with different names
        let config1 = AppConfig {
            name: "App One".to_string(),
            logo_url: None,
            primary_color: None,
        };
        let config2 = AppConfig {
            name: "App Two".to_string(),
            logo_url: None, 
            primary_color: None,
        };

        // Assert: Configs are not equal
        assert_ne!(config1, config2);
    }
}