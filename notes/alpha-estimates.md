# Alpha Milestone: "The Ghost Appliance" - Estimation

**Definition of a Living Document:** This document is subject to continuous updates and refinement as our understanding of the project evolves, new information emerges, or tasks are completed. It will be reviewed and updated at the beginning or end of relevant planning/progress sessions.

---

### **Goal:** Deliver a fully self-contained, end-to-end solution for adding multi-tenancy to a standard Ghost CMS installation, deployable with one click on AWS.

### **Total Estimated Timeline:** ~4 months (for a single full-time developer)

### **Total Estimated Person-Days:** ~81 person-days (realistic)

**Confidence Level:** Low. This is a greenfield project with significant technical complexity (unikernel orchestration, application-specific proxy logic, real-time dashboard). The estimate is a "best-case" scenario for a single developer. The real-world timeline could easily be 50-100% longer due to unforeseen challenges.

---

### **Estimation Breakdown by Phase:**

**Phase 0: Pre-flight & Setup (Hermit PoC)**

*   **Task:** Build and deploy a minimal "Hello, Hyper!" server as a Hermit unikernel.
*   **Purpose:** Technical de-risking and environment setup.
*   **Estimate:** **2 person-days.**

---

**Phase 1: Core Engine & CLI (The "Artisan" Tool)**

*   **Purpose:** Build the foundational control plane and its primary interface.
*   **Total Estimate:** **23-32 person-days (average 28 days).**

    *   **Task 4: Build the `naaas-server` Control Plane (Rust):**
        *   **Description:** Core HTTP API, tenant lifecycle management, hypervisor orchestration (starting/stopping/configuring unikernels).
        *   **Estimate:** 15-20 person-days.
    *   **Task 5: Build the `naaas-ctl` CLI (Rust):**
        *   **Description:** CLI framework, API client, core commands (`deploy`, `delete`, `list`), configuration management (`init`, `get`, `set`, `edit`).
        *   **Estimate:** 8-12 person-days.

---

**Phase 2: The Ghost Integration (The "Secret Sauce")**

*   **Purpose:** Implement the tenant shimunikernel with Ghost-specific logic and core features.
*   **Total Estimate:** **21-29 person-days (average 25 days).**

    *   **Task 6: Build the Tenant Shim Unikernel (Rust):**
        *   **Description:** Base HTTP proxying, TLS termination, `hermit-os/tokio` integration, Ghost-specific content isolation ("Tenant-as-a-Tag"), Ghost-specific user isolation (`naaas_tenant_id`), `/config` endpoint.
        *   **Estimate:** 15-20 person-days.
    *   **Task 7: Implement High-Value "Sticky" Features:**
        *   **Description:** Caching, Header Policy, Lightweight Logging built into the shim.
        *   **Estimate:** 6-9 person-days.

---

**Phase 3: The Appliance & Dashboard (The "Polished Product")**

*   **Purpose:** Create the user-friendly packaged experience.
*   **Total Estimate:** **21-31 person-days (average 26 days).**

    *   **Task 8: Build the V1 "Fleet View" Dashboard (React/Vue/etc.):**
        *   **Description:** Web frontend for status, metrics, and basic management, implementing the "Fleet View" vision.
        *   **Estimate:** 12-18 person-days.
    *   **Task 9: Package the AWS AMI:**
        *   **Description:** Automation for creating the AMI, including OS, hypervisor, `naaas-server`, and dashboard web server.
        *   **Estimate:** 4-6 person-days.
    *   **Task 10: Write Documentation & Finalize:**
        *   **Description:** User-facing documentation for the Alpha release, final testing.
        *   **Estimate:** 5-7 person-days.
