use crate::models::{Tenant, DeployRequest};
use anyhow::Result;
use hyper::{Body, Response, StatusCode};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::process::{Command, Stdio};
use tracing::{info, warn, error};
use uuid::Uuid;

/// Thread-safe store for managing tenant data
pub type TenantStore = Arc<Mutex<HashMap<String, Tenant>>>;

/// Deploy a new tenant unikernel
pub async fn handle_deploy(
    body: Body,
    store: TenantStore,
) -> Result<Response<Body>, anyhow::Error> {
    // Parse the JSON request body
    let bytes = hyper::body::to_bytes(body).await?;
    let deploy_req: DeployRequest = serde_json::from_slice(&bytes)?;
    
    // Validate the request data
    if let Err(validation_error) = deploy_req.validate() {
        warn!("Deploy request validation failed: {}", validation_error);
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(format!("{{\"error\": \"{}\"}}", validation_error)))?);
    }
    
    // Generate unique tenant ID and assign port
    let tenant_id = Uuid::new_v4().to_string();
    let port = deploy_req.get_port();
    
    info!("Deploying tenant: {} on port {}", deploy_req.name, port);
    
    // Launch the unikernel process
    match spawn_unikernel_process(&deploy_req, port).await {
        Ok(process_id) => {
            // Create and store the tenant
            let tenant = Tenant::new(
                tenant_id.clone(),
                deploy_req.name,
                port,
                Some(process_id),
                deploy_req.unikernel_path,
            );
            
            store.lock().unwrap().insert(tenant_id.clone(), tenant.clone());
            
            info!("Tenant {} deployed successfully with process ID {}", tenant_id, process_id);
            
            // Return the tenant data
            let response_json = serde_json::to_string(&tenant)?;
            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(Body::from(response_json))?)
        }
        Err(spawn_error) => {
            error!("Failed to spawn unikernel process: {}", spawn_error);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(format!("{{\"error\": \"Failed to start unikernel: {}\"}}", spawn_error)))?)
        }
    }
}

/// List all deployed tenants
pub async fn handle_list_tenants(store: TenantStore) -> Result<Response<Body>, anyhow::Error> {
    let tenants: Vec<Tenant> = store.lock().unwrap().values().cloned().collect();
    let response_json = serde_json::to_string(&tenants)?;
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(response_json))?)
}

/// Delete a specific tenant
pub async fn handle_delete_tenant(
    tenant_id: String,
    store: TenantStore,
) -> Result<Response<Body>, anyhow::Error> {
    let mut tenants = store.lock().unwrap();
    
    if let Some(tenant) = tenants.remove(&tenant_id) {
        info!("Stopping tenant: {}", tenant_id);
        
        // Attempt to kill the process
        if let Some(pid) = tenant.process_id {
            if let Err(kill_error) = terminate_process(pid) {
                warn!("Failed to terminate process {}: {}", pid, kill_error);
            }
        }
        
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from("{\"message\": \"Tenant deleted successfully\"}"))?)
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "application/json")
            .body(Body::from("{\"error\": \"Tenant not found\"}"))?)
    }
}

/// Health check endpoint
pub async fn handle_health() -> Result<Response<Body>, anyhow::Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from("{\"status\": \"healthy\", \"service\": \"naaas-server\"}"))?)
}

/// Build command arguments for unikernel process
pub fn build_unikernel_args(deploy_req: &DeployRequest, port: u16) -> Vec<String> {
    let mut cmd_args = vec![
        "--port".to_string(), 
        port.to_string(),
    ];
    
    // Add upstream URL if provided (Sprint 2 feature)
    if let Some(upstream) = &deploy_req.upstream_url {
        cmd_args.push("--upstream".to_string());
        cmd_args.push(upstream.clone());
    }
    
    // Add app config if provided
    if let Some(config) = &deploy_req.app_config {
        cmd_args.push("--config".to_string());
        cmd_args.push(config.clone());
    }
    
    cmd_args
}

/// Spawn a unikernel process (currently spawns regular binaries for Sprint 1)
async fn spawn_unikernel_process(
    deploy_req: &DeployRequest, 
    port: u16
) -> Result<u32, anyhow::Error> {
    let cmd_args = build_unikernel_args(deploy_req, port);
    
    // Spawn the process
    let cmd = Command::new(&deploy_req.unikernel_path)
        .args(&cmd_args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    
    let process_id = cmd.id();
    
    // Detach the process so it runs independently
    std::mem::forget(cmd);
    
    Ok(process_id)
}

/// Terminate a process by PID
fn terminate_process(pid: u32) -> Result<(), anyhow::Error> {
    Command::new("kill")
        .arg(pid.to_string())
        .output()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    /// Helper function to create a test tenant store
    fn create_test_store() -> TenantStore {
        Arc::new(Mutex::new(HashMap::new()))
    }
    
    /// Helper function to create a test tenant
    fn create_test_tenant(id: &str, name: &str) -> Tenant {
        Tenant::new(
            id.to_string(),
            name.to_string(),
            3001,
            Some(12345),
            "/test/path".to_string(),
        )
    }

    #[tokio::test]
    async fn test_handle_list_tenants_returns_empty_array_when_no_tenants() {
        // Setup: Empty tenant store
        let store = create_test_store();

        // Action: List tenants
        let response = handle_list_tenants(store).await.unwrap();

        // Assert: Response is OK with empty array
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
        
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let tenants: Vec<Tenant> = serde_json::from_slice(&body_bytes).unwrap();
        assert!(tenants.is_empty(), "Should return empty array when no tenants exist");
    }

    #[tokio::test]
    async fn test_handle_list_tenants_returns_all_stored_tenants() {
        // Setup: Store with multiple tenants
        let store = create_test_store();
        let tenant1 = create_test_tenant("id1", "tenant1");
        let tenant2 = create_test_tenant("id2", "tenant2");
        
        store.lock().unwrap().insert("id1".to_string(), tenant1.clone());
        store.lock().unwrap().insert("id2".to_string(), tenant2.clone());

        // Action: List tenants
        let response = handle_list_tenants(store).await.unwrap();

        // Assert: Response contains both tenants
        assert_eq!(response.status(), StatusCode::OK);
        
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let tenants: Vec<Tenant> = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(tenants.len(), 2, "Should return all stored tenants");
        
        // Check that both tenants are present (order may vary)
        let tenant_names: Vec<String> = tenants.iter().map(|t| t.name.clone()).collect();
        assert!(tenant_names.contains(&"tenant1".to_string()));
        assert!(tenant_names.contains(&"tenant2".to_string()));
    }

    #[tokio::test]
    async fn test_handle_delete_tenant_removes_existing_tenant() {
        // Setup: Store with one tenant
        let store = create_test_store();
        let tenant = create_test_tenant("test-id", "test-tenant");
        store.lock().unwrap().insert("test-id".to_string(), tenant);

        // Action: Delete the tenant
        let response = handle_delete_tenant("test-id".to_string(), store.clone()).await.unwrap();

        // Assert: Tenant is deleted successfully
        assert_eq!(response.status(), StatusCode::OK);
        assert!(store.lock().unwrap().is_empty(), "Tenant should be removed from store");
        
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert!(body_str.contains("deleted successfully"), "Response should confirm deletion");
    }

    #[tokio::test]
    async fn test_handle_delete_tenant_returns_not_found_for_missing_tenant() {
        // Setup: Empty store
        let store = create_test_store();

        // Action: Try to delete non-existent tenant
        let response = handle_delete_tenant("non-existent".to_string(), store).await.unwrap();

        // Assert: Returns 404 Not Found
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert!(body_str.contains("not found"), "Response should indicate tenant not found");
    }

    #[tokio::test]
    async fn test_handle_health_returns_healthy_status() {
        // Action: Call health endpoint
        let response = handle_health().await.unwrap();

        // Assert: Returns healthy status
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
        
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert!(body_str.contains("healthy"), "Response should indicate healthy status");
        assert!(body_str.contains("naaas-server"), "Response should identify the service");
    }

    #[tokio::test]
    async fn test_handle_deploy_validates_request_and_rejects_invalid_data() {
        // Setup: Store and invalid request (empty name)
        let store = create_test_store();
        let invalid_request = DeployRequest {
            name: "".to_string(), // Invalid: empty name
            unikernel_path: "/valid/path".to_string(),
            port: Some(3000),
            upstream_url: None,
            app_config: None,
        };
        let body = Body::from(serde_json::to_string(&invalid_request).unwrap());

        // Action: Try to deploy with invalid request
        let response = handle_deploy(body, store).await.unwrap();

        // Assert: Returns bad request with validation error
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert!(body_str.contains("error"), "Response should contain error message");
        assert!(body_str.contains("name"), "Error should mention the name field");
    }

    #[test]
    fn test_build_unikernel_args_with_basic_request() {
        // Setup: Basic deploy request with only required fields
        let request = DeployRequest {
            name: "test-app".to_string(),
            unikernel_path: "/path/to/unikernel".to_string(),
            port: None,
            upstream_url: None,
            app_config: None,
        };

        // Action: Build command arguments
        let args = build_unikernel_args(&request, 3001);

        // Assert: Contains port arguments only
        assert_eq!(args, vec!["--port", "3001"]);
    }

    #[test]
    fn test_build_unikernel_args_with_upstream_url() {
        // Setup: Deploy request with upstream URL
        let request = DeployRequest {
            name: "proxy-app".to_string(),
            unikernel_path: "/path/to/shim".to_string(),
            port: None,
            upstream_url: Some("http://localhost:2368".to_string()),
            app_config: None,
        };

        // Action: Build command arguments
        let args = build_unikernel_args(&request, 3002);

        // Assert: Contains port and upstream arguments
        assert_eq!(args, vec![
            "--port", "3002",
            "--upstream", "http://localhost:2368"
        ]);
    }

    #[test]
    fn test_build_unikernel_args_with_app_config() {
        // Setup: Deploy request with app config
        let request = DeployRequest {
            name: "config-app".to_string(),
            unikernel_path: "/path/to/app".to_string(),
            port: None,
            upstream_url: None,
            app_config: Some(r#"{"name":"Test App","color":"blue"}"#.to_string()),
        };

        // Action: Build command arguments
        let args = build_unikernel_args(&request, 4000);

        // Assert: Contains port and config arguments
        assert_eq!(args, vec![
            "--port", "4000",
            "--config", r#"{"name":"Test App","color":"blue"}"#
        ]);
    }

    #[test]
    fn test_build_unikernel_args_with_all_options() {
        // Setup: Deploy request with all optional fields
        let request = DeployRequest {
            name: "full-app".to_string(),
            unikernel_path: "/path/to/full-app".to_string(),
            port: Some(5000), // This will be overridden by the function parameter
            upstream_url: Some("https://api.example.com".to_string()),
            app_config: Some(r#"{"name":"Full App","theme":"dark","features":["auth","logs"]}"#.to_string()),
        };

        // Action: Build command arguments
        let args = build_unikernel_args(&request, 6000);

        // Assert: Contains all arguments in correct order
        assert_eq!(args, vec![
            "--port", "6000",
            "--upstream", "https://api.example.com",
            "--config", r#"{"name":"Full App","theme":"dark","features":["auth","logs"]}"#
        ]);
    }

    #[test]
    fn test_build_unikernel_args_with_complex_upstream_url() {
        // Setup: Deploy request with complex upstream URL
        let request = DeployRequest {
            name: "complex-proxy".to_string(),
            unikernel_path: "/path/to/proxy".to_string(),
            port: None,
            upstream_url: Some("https://cms.internal.company.com:8443/api/v2".to_string()),
            app_config: None,
        };

        // Action: Build command arguments
        let args = build_unikernel_args(&request, 7001);

        // Assert: Complex upstream URL is preserved exactly
        assert_eq!(args, vec![
            "--port", "7001",
            "--upstream", "https://cms.internal.company.com:8443/api/v2"
        ]);
    }

    #[test]
    fn test_build_unikernel_args_with_json_config_containing_quotes() {
        // Setup: Deploy request with complex JSON config
        let request = DeployRequest {
            name: "json-app".to_string(),
            unikernel_path: "/path/to/app".to_string(),
            port: None,
            upstream_url: None,
            app_config: Some(r#"{"title":"App \"with quotes\"","description":"Complex JSON"}"#.to_string()),
        };

        // Action: Build command arguments
        let args = build_unikernel_args(&request, 8000);

        // Assert: JSON with quotes is preserved
        assert_eq!(args, vec![
            "--port", "8000",
            "--config", r#"{"title":"App \"with quotes\"","description":"Complex JSON"}"#
        ]);
    }

    #[test]
    fn test_build_unikernel_args_port_override() {
        // Setup: Deploy request with port specified in request
        let request = DeployRequest {
            name: "port-test".to_string(),
            unikernel_path: "/test/app".to_string(),
            port: Some(9999), // Request specifies port 9999
            upstream_url: None,
            app_config: None,
        };

        // Action: Build args with different port parameter
        let args = build_unikernel_args(&request, 8888);

        // Assert: Function parameter overrides request port
        assert_eq!(args, vec!["--port", "8888"]);
    }
}