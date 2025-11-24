# Pluggable Architecture for Multi-Platform Deployment

This document outlines a modular, pluggable architecture for the `naaas-server` that allows the NAAAS project to target a specific platform like AWS initially, while leaving the door open for future support of other deployment environments like bare-metal, GCP, or Azure.

## The Core Philosophy

The key to portability is to separate the **generic orchestration logic** of NAAAS from the **platform-specific implementation details**. The `naaas-server` should not know or care if it's launching a unikernel via the AWS API or by executing a local shell command.

This is achieved by defining a clear `InfrastructureProvider` interface and building the first implementation for AWS.

---

## The `InfrastructureProvider` Interface

The `naaas-server`'s core logic will be written to interact with a generic `InfrastructureProvider` trait (in Rust terminology). This trait defines the set of capabilities that any underlying platform must provide.

This interface acts as a contract, abstracting away the specifics of the infrastructure.

### Example Interface Definition (Conceptual)

```rust
// A simplified representation of the provider trait

pub struct TenantDeployment {
    pub tenant_id: String,
    pub public_endpoint: String, // e.g., "https://tenant-a.com"
    pub internal_id: String,     // e.g., an AWS ARN or a local process ID
}

#[async_trait]
pub trait InfrastructureProvider {
    /// Returns the name of the provider (e.g., "aws-fargate", "local-firecracker")
    fn name(&self) -> &str;

    /// Provisions the necessary infrastructure for a tenant's unikernel.
    /// This is the main creation method.
    ///
    /// # Arguments
    /// * `config` - The declarative configuration for the tenant.
    ///
    /// # Returns
    /// A `Result` containing the `TenantDeployment` details on success.
    async fn provision_tenant(&self, config: &TenantConfig) -> Result<TenantDeployment, Error>;

    /// Destroys all infrastructure associated with a tenant.
    ///
    /// # Arguments
    /// * `deployment` - The deployment information to be destroyed.
    async fn destroy_tenant(&self, deployment: &TenantDeployment) -> Result<(), Error>;

    /// Retrieves the current status and health of a tenant's deployment.
    ///
    /// # Arguments
    /// * `deployment` - The deployment to check.
    async fn get_tenant_status(&self, deployment: &TenantDeployment) -> Result<DeploymentStatus, Error>;

    /// Retrieves logs for a specific tenant deployment.
    ///
    /// # Arguments
    /// * `deployment` - The deployment to fetch logs for.
    /// * `options` - Time range, number of lines, etc.
    async fn get_tenant_logs(&self, deployment: &TenantDeployment, options: &LogOptions) -> Result<Vec<String>, Error>;
}
```

---

## Initial Implementation: `AWSProvider`

To start, we will implement a single, concrete provider for our chosen AWS architecture (e.g., using EC2+Firecracker or Fargate).

### Example `AWSProvider` Implementation (Conceptual)

```rust
pub struct AWSProvider {
    // Clients for AWS services
    ec2_client: aws_sdk_ec2::Client,
    alb_client: aws_sdk_elasticloadbalancingv2::Client,
    iam_client: aws_sdk_iam::Client,
    // ... and so on
}

#[async_trait]
impl InfrastructureProvider for AWSProvider {
    async fn provision_tenant(&self, config: &TenantConfig) -> Result<TenantDeployment, Error> {
        // This method would contain all the AWS API calls to:
        // 1. Create a dedicated IAM Role for the unikernel.
        // 2. Create and register a target in the ALB Target Group.
        // 3. Add a rule to the ALB listener to route `tenant-a.com` to the new target.
        // 4. Use AWS Certificate Manager to ensure a valid TLS certificate is attached.
        // 5. Launch an EC2 instance or Fargate task to run the unikernel.
        // 6. Return the resulting deployment details.
        // ... implementation ...
    }

    async fn destroy_tenant(&self, deployment: &TenantDeployment) -> Result<(), Error> {
        // Make AWS API calls to reverse the steps in `provision_tenant`.
        // ... implementation ...
    }

    // ... other method implementations ...
}
```

## Future Implementations: `BareMetalProvider`

This architecture makes it clear how to add support for a different platform. A contributor wanting bare-metal support would create a `BareMetalProvider`.

### Example `BareMetalProvider` (Conceptual)

```rust
pub struct BareMetalProvider {
    // Path to the Firecracker binary, network configuration, etc.
    hypervisor_config: HypervisorConfig,
}

#[async_trait]
impl InfrastructureProvider for BareMetalProvider {
    async fn provision_tenant(&self, config: &TenantConfig) -> Result<TenantDeployment, Error> {
        // This method would contain all the local orchestration logic:
        // 1. Create a network TAP device for the new microVM.
        // 2. Generate a Firecracker configuration file.
        // 3. Execute the `firecracker` process locally.
        // 4. Update a local reverse proxy (like Nginx or HAProxy) to route traffic.
        // 5. Return the deployment details (e.g., with the process ID as the `internal_id`).
        // ... implementation ...
    }

    // ... other method implementations ...
}
```

## Benefits of This Architecture

*   **Pragmatism and Focus:** It allows us to focus entirely on building a best-in-class experience on a single platform (AWS) first, without boiling the ocean.
*   **Clear Extensibility:** It provides a clear, well-defined "seam" in the application for adding new platforms. The work required to support a new platform is isolated to implementing a single trait.
*   **Testability:** The core `naaas-server` logic can be tested independently by using a mock `InfrastructureProvider`, separating the business logic from the infrastructure implementation.
*   **Future-Proofing:** It prevents us from being permanently locked into a single platform's way of doing things and allows the project to evolve as new technologies or platforms emerge.
