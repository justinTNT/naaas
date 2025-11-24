# NAAAS MVP Engineering Requirements (Thin Wrapper Edition)

This document specifies the technical requirements for the NAAAS Alpha Milestone, focused on creating a "Thin Infrastructure Wrapper."

---
## 1. Core Platform Requirements

*   **1.1. Target Monolith:** The platform MUST be tested against a standard **Ghost CMS** installation to prove its ability to wrap a real-world application.
*   **1.2. Application Agnosticism:** The platform's features MUST be application-agnostic. No business logic or application-specific parsing is permitted in the shim.
*   **1.3. Configuration Model:** All tenant configuration MUST be managed via declarative JSON files.
*   **1.4. Execution Environment:** Each tenant shim MUST be deployed as a fully isolated unikernel on a KVM/Firecracker hypervisor.

---
## 2. `naaas-server` (Control Plane) Requirements

*   **2.1. API:** MUST expose a secure, internal HTTP API for all platform operations (tenant lifecycle, configuration).
*   **2.2. Tenant Management:** MUST handle the lifecycle (create, read, update, delete) of tenants based on their declarative configuration files.
*   **2.3. Orchestration:** MUST be able to build the Rust shim project into a unikernel image, deploy it to the hypervisor, inject tenant-specific configuration (e.g., upstream URL, `app_config` JSON) at boot time, and manage its lifecycle.

---
## 3. `naaas-ctl` (CLI) Requirements

*   **3.1. API Client:** MUST act as a client to the `naaas-server`'s HTTP API.
*   **3.2. Core Commands:** MUST provide commands for core lifecycle operations (`deploy`, `delete`, `list`).
*   **3.3. Configuration Management:** MUST provide a full suite of commands to manage tenant JSON configuration files, including an interactive `init` wizard and scriptable `get`/`set` commands.

---
## 4. Tenant Shim Unikernel Requirements (The "Thin Wrapper")

The shim's responsibilities are strictly limited to the infrastructure layer.

### 4.1. Core Functionality
*   **4.1.1. Transparent Proxying:** MUST act as a transparent reverse proxy for all HTTP/HTTPS traffic to the configured upstream URL. It MUST NOT inspect or modify application-specific headers (like `Authorization`) or request/response bodies.
*   **4.1.2. Opaque Configuration Serving:** MUST expose a `GET /config` endpoint to serve the raw, opaque `app_config` JSON blob that was provided at deploy time.

### 4.2. Infrastructure Features
*   **4.2.1. TLS Termination:** MUST handle TLS termination and automated certificate management for the tenant's public domain.
*   **4.2.2. Rate Limiting:** MUST enforce per-tenant request rate limits based on rules in the `infra_config`.
*   **4.2.3. Lightweight Logging:** MUST emit structured JSON access logs to `stdout` for every request, containing at a minimum: timestamp, tenant ID, source IP, method, path, status code, and latency.

---
## 5. V1 Admin Dashboard Requirements

*   **5.1. API Client:** The dashboard MUST operate as a pure client of the `naaas-server`'s HTTP API.
*   **5.2. "Fleet View":** The primary interface MUST be a visual grid of "cards" representing each deployed tenant.
*   **5.3. Dynamic Branding:** Each card's styling (logo, colors) MUST be derived from that tenant's publicly accessible `/config` endpoint.
*   **5.4. At-a-Glance Telemetry:** Each card MUST display a real-time status indicator and a sparkline graph of recent activity.
*   **5.5. Live Updates:** The dashboard MUST update its view in near real-time.
