//! NAAAS Server - Control plane for managing tenant unikernels
//! 
//! This library provides the core functionality for the NAAAS server,
//! including tenant management, HTTP API handlers, and data models.

pub mod models;
pub mod handlers;

pub use models::{Tenant, DeployRequest};
pub use handlers::{TenantStore, handle_deploy, handle_list_tenants, handle_delete_tenant, handle_health};