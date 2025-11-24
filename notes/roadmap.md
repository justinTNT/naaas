# NAAAS Development Roadmap (Thin Wrapper Edition)

This document outlines the development strategy for NAAAS, focusing on delivering the "Alpha Milestone" with a strict "thin infrastructure wrapper" scope.

---
## The Alpha Milestone: "The Ghost Appliance"

Our primary goal is to deliver a self-contained appliance that demonstrates the value of wrapping a real-world application (Ghost CMS) to provide infrastructure-level "enterprise" features.

**Alpha Deliverables:**
1.  A public AWS AMI named "NAAAS for Ghost."
2.  The AMI will contain the pre-configured `naaas-server` control plane and a web server hosting the V1 Admin Dashboard.
3.  The `naaas-ctl` CLI binary for remote management and automation.
4.  Documentation explaining how to launch the AMI and use it to proxy a Ghost instance, highlighting the infrastructure features provided.

---
## V1 Web Dashboard Vision: "Fleet View"

The dashboard's purpose is ongoing monitoring of the deployed shims.
*   **Visual Representation:** A grid of "cards" for each tenant.
*   **Branded Icons:** Cards will be styled using the tenant's `app_config` (logo, primary color) fetched from the shim's `/config` endpoint.
*   **At-a-Glance Telemetry:** Each card will display real-time status, a sparkline for recent traffic, and key deployment details.

---
## Development Methodology: Iterative Vertical Slices

### Phase 0: Pre-flight & Setup
*   **Task 1: Hermit "Hello, Hyper!" PoC:** Validate the core technology stack by building and running a minimal Hermit unikernel. (Reference: `hermit_poc_plan.md`)
*   **Task 2: Local Ghost CMS Setup:** Have a local, standard Ghost CMS instance running as our target monolith.

### Sprint 1: The "Launch Unikernel" Slice
*   **Goal:** `naaas-ctl` can command the `naaas-server` to launch the "Hello, Hyper!" unikernel.
*   **Key Components:** Minimal `naaas-server` with a `/deploy` API; minimal `naaas-ctl` with a `deploy` command; integration with the hypervisor to start a process.
*   **Feedback:** A working `naaas-ctl deploy` command that successfully launches a unikernel.

### Sprint 2: The "Transparent Proxy" Slice
*   **Goal:** Deploy a `naaas-shim` that transparently proxies all HTTP traffic to the local Ghost instance.
*   **Key Components:** `naaas-shim` with basic reverse proxy logic; `naaas-server` updated to configure the shim with the upstream Ghost URL.
*   **Test:** `naaas-ctl deploy` the proxy, `curl` the shim's port, and verify it returns Ghost's homepage.
*   **Feedback:** A working proxy that successfully wraps a real-world application.

### Sprint 3: The "Core Infrastructure Features" Slice
*   **Goal:** Implement the core infrastructure features in the `naaas-shim`.
*   **Key Components:** Enhance the `naaas-shim` and the `naaas-server` configuration options.
*   **Workflow (Iterative, TDD):**
    1.  **TLS Termination:** Configure a shim with a domain and certificate; test HTTPS access.
    2.  **Rate Limiting:** Configure a rate limit; write a test that asserts a `429 Too Many Requests` response after the limit is exceeded.
    3.  **Lightweight Logging:** Verify that `curl`ing the shim produces a structured JSON log on the server's `stdout`.
    4.  **Config Serving:** Deploy a shim with a sample `app_config.json`; test that a `GET /config` request to the shim returns the correct JSON.
*   **Feedback:** A shim that provides the full suite of our defined infrastructure-level features.

### Sprint 4: The "Dashboard & Packaging" Slice
*   **Goal:** Build the V1 Admin Dashboard and package the AWS AMI.
*   **Key Components:** Dashboard frontend; AMI automation scripts.
*   **Workflow:**
    1.  Develop the V1 "Fleet View" Admin Dashboard.
    2.  Integrate the dashboard with the `naaas-server` API.
    3.  Create the automated scripts (e.g., using Packer) for building the final AWS AMI.
    4.  Generate and test the first Alpha AMI.
*   **Feedback:** A deployable Ghost Appliance on AWS with a functional dashboard.

### Sprint 5: Documentation & Final Testing
*   **Goal:** Complete user-facing documentation and final integration testing.
*   **Deliverable:** Comprehensive Alpha release documentation and a finalized AMI.