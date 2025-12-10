use serde::{Deserialize, Serialize};

/// Represents a deployed tenant unikernel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub status: String,
    pub port: u16,
    pub process_id: Option<u32>,
    pub unikernel_path: String,
}

/// Request structure for deploying a new tenant
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeployRequest {
    pub name: String,
    pub unikernel_path: String,
    pub port: Option<u16>,
    pub upstream_url: Option<String>,
    pub app_config: Option<String>,
}

impl Tenant {
    /// Create a new tenant with the given parameters
    pub fn new(
        id: String,
        name: String,
        port: u16,
        process_id: Option<u32>,
        unikernel_path: String,
    ) -> Self {
        Self {
            id,
            name,
            status: "running".to_string(),
            port,
            process_id,
            unikernel_path,
        }
    }

    /// Check if this tenant is considered active (has a process ID)
    pub fn is_active(&self) -> bool {
        self.process_id.is_some() && self.status == "running"
    }
}

impl DeployRequest {
    /// Validate that the deploy request has all required fields
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Tenant name cannot be empty".to_string());
        }

        if self.unikernel_path.trim().is_empty() {
            return Err("Unikernel path cannot be empty".to_string());
        }

        // Check port range if specified
        if let Some(port) = self.port {
            if port < 1024 {
                return Err("Port must be between 1024 and 65535".to_string());
            }
        }

        Ok(())
    }

    /// Get the port to use, either from request or default
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(3001)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_creation_with_all_fields() {
        // Setup: Create a tenant with all required data
        let tenant = Tenant::new(
            "test-id".to_string(),
            "test-tenant".to_string(),
            3001,
            Some(1234),
            "/path/to/unikernel".to_string(),
        );

        // Assert: All fields are set correctly
        assert_eq!(tenant.id, "test-id");
        assert_eq!(tenant.name, "test-tenant");
        assert_eq!(tenant.port, 3001);
        assert_eq!(tenant.process_id, Some(1234));
        assert_eq!(tenant.unikernel_path, "/path/to/unikernel");
        assert_eq!(tenant.status, "running");
    }

    #[test]
    fn test_tenant_is_active_when_has_process_id_and_running() {
        // Setup: Create an active tenant
        let tenant = Tenant::new(
            "test-id".to_string(),
            "test-tenant".to_string(),
            3001,
            Some(1234), // Has process ID
            "/path/to/unikernel".to_string(),
        );

        // Assert: Tenant is considered active
        assert!(tenant.is_active(), "Tenant with process ID and running status should be active");
    }

    #[test]
    fn test_tenant_is_not_active_when_no_process_id() {
        // Setup: Create a tenant without process ID
        let tenant = Tenant::new(
            "test-id".to_string(),
            "test-tenant".to_string(),
            3001,
            None, // No process ID
            "/path/to/unikernel".to_string(),
        );

        // Assert: Tenant is not active
        assert!(!tenant.is_active(), "Tenant without process ID should not be active");
    }

    #[test]
    fn test_deploy_request_validates_successfully_with_all_fields() {
        // Setup: Create a valid deploy request
        let request = DeployRequest {
            name: "valid-tenant".to_string(),
            unikernel_path: "/valid/path".to_string(),
            port: Some(3000),
            upstream_url: Some("http://localhost:2368".to_string()),
            app_config: Some("{}".to_string()),
        };

        // Action: Validate the request
        let result = request.validate();

        // Assert: Validation passes
        assert!(result.is_ok(), "Valid request should pass validation");
    }

    #[test]
    fn test_deploy_request_fails_validation_with_empty_name() {
        // Setup: Create request with empty name
        let request = DeployRequest {
            name: "   ".to_string(), // Empty/whitespace name
            unikernel_path: "/valid/path".to_string(),
            port: None,
            upstream_url: None,
            app_config: None,
        };

        // Action: Validate the request
        let result = request.validate();

        // Assert: Validation fails with helpful message
        assert!(result.is_err(), "Empty name should fail validation");
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("name cannot be empty"), "Error should mention empty name");
    }

    #[test]
    fn test_deploy_request_fails_validation_with_empty_unikernel_path() {
        // Setup: Create request with empty unikernel path
        let request = DeployRequest {
            name: "valid-name".to_string(),
            unikernel_path: "".to_string(), // Empty path
            port: None,
            upstream_url: None,
            app_config: None,
        };

        // Action: Validate the request
        let result = request.validate();

        // Assert: Validation fails with helpful message
        assert!(result.is_err(), "Empty unikernel path should fail validation");
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("path cannot be empty"), "Error should mention empty path");
    }

    #[test]
    fn test_deploy_request_fails_validation_with_invalid_port() {
        // Setup: Create request with invalid port
        let request = DeployRequest {
            name: "valid-name".to_string(),
            unikernel_path: "/valid/path".to_string(),
            port: Some(80), // Port too low
            upstream_url: None,
            app_config: None,
        };

        // Action: Validate the request
        let result = request.validate();

        // Assert: Validation fails with helpful message
        assert!(result.is_err(), "Low port should fail validation");
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("between 1024 and 65535"), "Error should mention port range");
    }

    #[test]
    fn test_deploy_request_get_port_returns_specified_port() {
        // Setup: Create request with specific port
        let request = DeployRequest {
            name: "test".to_string(),
            unikernel_path: "/path".to_string(),
            port: Some(4000),
            upstream_url: None,
            app_config: None,
        };

        // Action: Get the port
        let port = request.get_port();

        // Assert: Returns the specified port
        assert_eq!(port, 4000, "Should return the specified port");
    }

    #[test]
    fn test_deploy_request_get_port_returns_default_when_none() {
        // Setup: Create request without port
        let request = DeployRequest {
            name: "test".to_string(),
            unikernel_path: "/path".to_string(),
            port: None, // No port specified
            upstream_url: None,
            app_config: None,
        };

        // Action: Get the port
        let port = request.get_port();

        // Assert: Returns the default port
        assert_eq!(port, 3001, "Should return default port when none specified");
    }
}