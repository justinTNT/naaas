use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Tenant data structure (matches server response)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub status: String,
    pub port: u16,
    pub process_id: Option<u32>,
    pub unikernel_path: String,
}

/// Deploy request structure (matches server expectations)
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct DeployRequest {
    pub name: String,
    pub unikernel_path: String,
    pub port: Option<u16>,
    pub upstream_url: Option<String>,
    pub app_config: Option<String>,
}

/// HTTP client for communicating with NAAAS server
pub struct NaaasClient {
    base_url: String,
    client: reqwest::Client,
}

impl NaaasClient {
    /// Create a new NAAAS client with the given server URL
    pub fn new(server_url: String) -> Self {
        Self {
            base_url: server_url,
            client: reqwest::Client::new(),
        }
    }

    /// Deploy a new tenant
    pub async fn deploy_tenant(&self, request: DeployRequest) -> Result<Tenant> {
        let url = format!("{}/deploy", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let tenant: Tenant = response.json().await?;
            Ok(tenant)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Deploy failed: {}", error_text))
        }
    }

    /// List all deployed tenants
    pub async fn list_tenants(&self) -> Result<Vec<Tenant>> {
        let url = format!("{}/tenants", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let tenants: Vec<Tenant> = response.json().await?;
            Ok(tenants)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("List tenants failed: {}", error_text))
        }
    }

    /// Delete a specific tenant
    pub async fn delete_tenant(&self, tenant_id: &str) -> Result<()> {
        let url = format!("{}/tenants/{}", self.base_url, tenant_id);
        
        let response = self.client
            .delete(&url)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else if response.status() == 404 {
            Err(anyhow::anyhow!("Tenant not found: {}", tenant_id))
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Delete failed: {}", error_text))
        }
    }

    /// Check server health
    pub async fn check_health(&self) -> Result<()> {
        let url = format!("{}/health", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Health check failed: server not responding properly"))
        }
    }

    /// Get the base URL for this client
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}

impl DeployRequest {
    /// Create a new deploy request with the required fields
    pub fn new(name: String, unikernel_path: String) -> Self {
        Self {
            name,
            unikernel_path,
            port: None,
            upstream_url: None,
            app_config: None,
        }
    }

    /// Set the port for this deploy request
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Set the upstream URL for this deploy request
    pub fn with_upstream_url(mut self, upstream_url: String) -> Self {
        self.upstream_url = Some(upstream_url);
        self
    }

    /// Set the app config for this deploy request
    pub fn with_app_config(mut self, app_config: String) -> Self {
        self.app_config = Some(app_config);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naaas_client_creation_stores_base_url_correctly() {
        // Setup: Create client with test URL
        let test_url = "http://test-server:9000".to_string();
        let client = NaaasClient::new(test_url.clone());

        // Assert: Base URL is stored correctly
        assert_eq!(client.get_base_url(), &test_url);
    }

    #[test]
    fn test_deploy_request_builder_pattern_works_correctly() {
        // Setup: Build deploy request using builder pattern
        let request = DeployRequest::new(
            "test-tenant".to_string(),
            "/path/to/unikernel".to_string()
        )
        .with_port(4000)
        .with_upstream_url("http://upstream:2368".to_string())
        .with_app_config("{\"name\": \"Test\"}".to_string());

        // Assert: All fields are set correctly
        assert_eq!(request.name, "test-tenant");
        assert_eq!(request.unikernel_path, "/path/to/unikernel");
        assert_eq!(request.port, Some(4000));
        assert_eq!(request.upstream_url, Some("http://upstream:2368".to_string()));
        assert_eq!(request.app_config, Some("{\"name\": \"Test\"}".to_string()));
    }

    #[test]
    fn test_deploy_request_new_creates_minimal_request() {
        // Setup: Create minimal deploy request
        let request = DeployRequest::new(
            "minimal-tenant".to_string(),
            "/minimal/path".to_string()
        );

        // Assert: Required fields are set, optional fields are None
        assert_eq!(request.name, "minimal-tenant");
        assert_eq!(request.unikernel_path, "/minimal/path");
        assert_eq!(request.port, None);
        assert_eq!(request.upstream_url, None);
        assert_eq!(request.app_config, None);
    }

    #[test]
    fn test_tenant_struct_can_be_serialized_and_deserialized() {
        // Setup: Create a tenant with test data
        let original_tenant = Tenant {
            id: "test-id".to_string(),
            name: "test-tenant".to_string(),
            status: "running".to_string(),
            port: 3001,
            process_id: Some(12345),
            unikernel_path: "/path/to/unikernel".to_string(),
        };

        // Action: Serialize to JSON and back
        let json_string = serde_json::to_string(&original_tenant).unwrap();
        let deserialized_tenant: Tenant = serde_json::from_str(&json_string).unwrap();

        // Assert: Serialization roundtrip preserves all data
        assert_eq!(deserialized_tenant, original_tenant);
        assert_eq!(deserialized_tenant.id, "test-id");
        assert_eq!(deserialized_tenant.name, "test-tenant");
        assert_eq!(deserialized_tenant.status, "running");
        assert_eq!(deserialized_tenant.port, 3001);
        assert_eq!(deserialized_tenant.process_id, Some(12345));
        assert_eq!(deserialized_tenant.unikernel_path, "/path/to/unikernel");
    }

    #[test]
    fn test_deploy_request_can_be_serialized_to_expected_json() {
        // Setup: Create a deploy request with all fields
        let request = DeployRequest::new(
            "json-test".to_string(),
            "/json/path".to_string()
        )
        .with_port(5000)
        .with_upstream_url("http://json-upstream".to_string());

        // Action: Serialize to JSON
        let json_value: serde_json::Value = serde_json::to_value(&request).unwrap();

        // Assert: JSON contains expected fields and values
        assert_eq!(json_value["name"], "json-test");
        assert_eq!(json_value["unikernel_path"], "/json/path");
        assert_eq!(json_value["port"], 5000);
        assert_eq!(json_value["upstream_url"], "http://json-upstream");
        assert!(json_value["app_config"].is_null());
    }
}