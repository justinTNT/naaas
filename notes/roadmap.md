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
## V1 Web Dashboard Vision: "Fleet View" (Separate Webapp)

**Architecture Decision:** The dashboard will be a **separate frontend application** consuming `naaas-server` APIs, not embedded web UI.

**Dashboard Purpose:** Ongoing monitoring of the deployed shims through a clean web interface.

**Technical Approach:**
*   **Microservice Architecture:** Dashboard webapp ↔ `naaas-server` (Go) APIs ↔ Unikernel fleet
*   **API-First Design:** All dashboard features exposed as JSON APIs for reusability
*   **Technology Flexibility:** Frontend can use React/Vue/Svelte without affecting control plane
*   **Independent Deployment:** Dashboard and server can be developed, deployed, and scaled independently

**UI Vision:**
*   **Visual Representation:** A grid of "cards" for each tenant
*   **Branded Icons:** Cards styled using tenant's `app_config` (logo, primary color) fetched from shim's `/config` endpoint
*   **At-a-Glance Telemetry:** Each card displays real-time status, sparkline for recent traffic, key deployment details

---
## Current Architecture Decisions

### **Technology Stack (Post-Sprint 2)**
- **`naaas-server` (Control Plane):** Go - Fleet orchestration, API serving, process management
- **`naaas-shim` (Data Plane):** Rust - Performance-critical HTTP reverse proxy  
- **`naaas-ctl` (CLI):** Go - Single binary distribution, cross-platform compatibility
- **Dashboard:** Separate frontend webapp consuming `naaas-server` APIs

### **Design Principles**
- **Control vs Data Plane Separation:** Server handles orchestration, shims handle traffic
- **API-First Architecture:** All features exposed as JSON APIs for flexibility
- **Single Binary Deployment:** Both CLI and server deploy as static binaries
- **Zero Application Modification:** Transparent wrapping of monolithic applications

---
## Development Methodology: Iterative Vertical Slices

### Phase 0: Pre-flight & Setup ✅ COMPLETED
*   ✅ **Technology Validation:** Successfully pivoted from Hermit to Unikraft, deployed production HTTP server on AWS
*   ✅ **Target Application:** Ghost CMS validated as target monolith via integration testing  
*   ✅ **Infrastructure Proven:** Working unikernel at http://3.1.210.183:3000 demonstrates technology stack

### Sprint 1: The "Launch Unikernel" Slice ✅ COMPLETED  
*   **Goal:** `naaas-ctl` can command the `naaas-server` to launch the "Hello, Hyper!" unikernel.
*   **Key Components:** Minimal `naaas-server` with a `/deploy` API; minimal `naaas-ctl` with a `deploy` command; integration with the hypervisor to start a process.
*   **Status:** ✅ **Completed** - Working deployment pipeline with comprehensive testing (28 unit tests)
*   **Architecture Decision:** `naaas-server` will be **rewritten in Go** (better fit for orchestration/control plane work)
*   **Feedback:** ✅ Working `naaas-ctl deploy` command successfully launches unikernels

### Sprint 2: The "Transparent Proxy" Slice ✅ COMPLETED
*   **Goal:** Deploy a `naaas-shim` that transparently proxies all HTTP traffic to the local Ghost instance.
*   **Key Components:** `naaas-shim` with basic reverse proxy logic; `naaas-server` updated to configure the shim with the upstream Ghost URL.
*   **Status:** ✅ **Completed** - End-to-end transparent proxying with comprehensive testing (+22 unit tests, 54 total)
*   **Architecture Validated:** "Thin infrastructure wrapper" approach proven with Ghost CMS integration
*   **Test:** ✅ `naaas-ctl deploy` → `naaas-server` → `naaas-shim` → Ghost CMS working flawlessly
*   **Feedback:** ✅ Working proxy successfully wraps real-world application with zero modification required

### Sprint 3: The "Core Infrastructure Features" Slice
*   **Goal:** Implement the core infrastructure features in the `naaas-shim`.
*   **Key Components:** Enhance the `naaas-shim` and the `naaas-server` configuration options.
*   **Workflow (Iterative, TDD):**
    1.  **TLS Termination:** Configure a shim with client-provided certificate files; test HTTPS access.
    2.  **Rate Limiting:** Configure a rate limit; write a test that asserts a `429 Too Many Requests` response after the limit is exceeded.
    3.  **Lightweight Logging:** Verify that `curl`ing the shim produces a structured JSON log on the server's `stdout`.
    4.  **Config Serving:** Deploy a shim with a sample `app_config.json`; test that a `GET /config` request to the shim returns the correct JSON.
*   **Feedback:** A shim that provides the full suite of our defined infrastructure-level features.

### Sprint 4: The "Dashboard & Packaging" Slice
*   **Goal:** Build the V1 Admin Dashboard as separate webapp and package the AWS AMI.
*   **Key Components:** Standalone dashboard frontend; Enhanced `naaas-server` APIs; AMI automation scripts.
*   **Workflow:**
    1.  **Enhance `naaas-server` APIs** for dashboard consumption (tenant metrics, health data, configuration endpoints)
    2.  **Develop separate dashboard webapp** with "Fleet View" interface
    3.  **API Integration** between dashboard frontend and `naaas-server` backend
    4.  **Create AMI automation scripts** (e.g., using Packer) that include both server and dashboard
    5.  **Generate and test the first Alpha AMI** with full stack
*   **Architecture:** Dashboard webapp + `naaas-server` (Go) + `naaas-shim` (Rust) fleet
*   **Feedback:** A deployable Ghost Appliance on AWS with functional separated dashboard architecture.

### Sprint 5: Documentation & Final Testing
*   **Goal:** Complete user-facing documentation and final integration testing.
*   **Deliverable:** Comprehensive Alpha release documentation and a finalized AMI.