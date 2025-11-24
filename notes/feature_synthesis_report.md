# NAAAS Feature Synthesis Report

This document synthesizes the findings from the in-depth analysis of 11 open-source applications (Ghost, WordPress, Directus, SuiteCRM, Invoice Ninja, Redmine, Gitea, osTicket, Chatwoot, PrestaShop, and Metabase). The goal is to create a prioritized, data-driven list of potential NAAAS features.

---
## Methodology

Each candidate was analyzed for its architecture, data model, and multi-tenancy challenges. Potential NAAAS interventions were identified and then grouped into common features. Each feature is rated by its applicability across the 11 candidates and its estimated impact.

*   **Impact Scale:**
    *   **High:** Solves a critical, complex problem or provides a massive, obvious benefit (e.g., provides tenancy where none exists, fixes a major security issue, gives a 10x performance boost).
    *   **Medium:** Solves a significant operational headache or provides a major convenience (e.g., simplifies a complex configuration task, provides good observability).
    *   **Low:** A "nice-to-have" feature that adds polish but isn't a primary driver for adoption.

---
## Feature Breakdown

### Group 1: Core Infrastructure & "Enterprise Theater"

These are the foundational, application-agnostic features that provide immediate value to almost any web application.

| Feature | Description | Applicability | Impact | Candidates Exhibiting Need |
| :--- | :--- | :--- | :--- | :--- |
| **TLS Termination** | Automatically provisions/renews and terminates HTTPS for a tenant's custom domain. | 11/11 | **High** | All |
| **Transparent Proxying**| Reliably forward HTTP/S traffic to the upstream monolith. This is the baseline function. | 11/11 | **High** | All |
| **Lightweight Logging** | Emits structured (JSON) access logs to `stdout` for all tenant traffic. | 11/11 | **High** | All |
| **Rate Limiting** | Enforces per-tenant request limits to protect the upstream monolith from abuse. | 11/11 | **High** | All |
| **Instance Orchestration**| Radically simplifies the deployment of multiple, separate application instances. | 7/11 | **High** | SuiteCRM, Invoice Ninja, Redmine, Gitea, osTicket, Chatwoot, PrestaShop |
| **TCP Proxying** | Proxies raw TCP traffic, not just HTTP. | 2/11 | **Medium**| Gitea (for SSH), other non-HTTP services |

*Note on **Instance Orchestration**: This is the primary value for the "Fortress" and "API-First" archetypes. For these, its impact is **Critical**. It is the core of the "Multi-Instance Manager" use case.

*Note on **TCP Proxying**: While only directly required by Gitea in our list, this capability represents a significant expansion of the platform's utility beyond web apps. It is likely a post-MVP feature. A similar argument applies to **WebSocket Proxying** (required by Chatwoot).

---
### Group 2: Branding & Configuration (The "White-Labeling" Toolkit)

These features focus on solving the universal pain point of customizing the look and feel for each tenant.

| Feature | Description | Applicability | Impact | Candidates Exhibiting Need |
| :--- | :--- | :--- | :--- | :--- |
| **Opaque Config Serving** | Serves a tenant-specific, arbitrary JSON blob from a `/config` endpoint. | 11/11 | **High** | All (for branding frontends, passing variables, etc.) |
| **Static Content Serving** | Serves a directory of static files (CSS, JS, images) from the shim. | 8/11 | **High** | WordPress, SuiteCRM, Redmine, Gitea, osTicket, Metabase, PrestaShop, Directus |

*Note on **Static Content Serving**: This has two main use cases: (1) Providing a full frontend for headless APIs (Directus), and (2) Injecting custom CSS/JS to override and "white-label" the UI of monoliths with complex theming (SuiteCRM, osTicket, Metabase). This makes it a very high-impact feature.

---
### Group 3: Performance Enhancement

| Feature | Description | Applicability | Impact | Candidates Exhibiting Need |
| :--- | :--- | :--- | :--- | :--- |
| **Edge Caching** | Caches responses from specific upstream endpoints based on declarative rules. | 11/11 | **High** | All (especially PrestaShop, WordPress, Ghost) |

---
### Group 4: "Intelligent Wrapper" Application-Aware Features

These are the more advanced, "smart" interventions for applications whose APIs allow for them.

| Feature | Description | Applicability | Impact | Candidates Exhibiting Need |
| :--- | :--- | :--- | :--- | :--- |
| **Read Filtering** | Transforms a generic `GET` request into a filtered API call (e.g., `?filter=...`). | 3/11 | **High** | Ghost, WordPress, Directus |
| **Write Stamping** | Injects a `tenant_id` or tag into the body of a `POST`/`PUT` request. | 3/11 | **High** | Ghost, WordPress, Directus |
| **Auth Adaptation** | Bridges a modern auth scheme (e.g., OAuth2) to a legacy upstream auth method (e.g., API key in header). | 4/11 | **Medium**| Metabase, Redmine, Gitea, osTicket |
| **Orchestrate Native Tenancy** | Uses the NAAAS config to automate the setup of a capable app's own complex multi-tenancy features via its admin API. | 1/11 | **High** | Metabase |

*Note on **Read Filtering/Write Stamping**: This pattern is the key to the "Content-Hackable" archetype. For these apps, this feature is **Critical** as it's the core mechanism for providing data isolation on a single instance.

---
## Conclusion: Summary of Findings

1.  **Universal Features:** A core set of features are valuable to **every single candidate**: TLS, Proxying, Logging, Rate Limiting, Caching, and Config Serving. These form the bedrock of our platform.
2.  **The Great Divide:** The most significant finding is the split between applications that can be "sliced" at the data layer (Ghost, WordPress, Directus) and those that require separate instances (everyone else). This confirms that NAAAS needs to be good at both **intelligent wrapping** and **multi-instance management**.
3.  **High-Impact Niches:** Certain features, while not universally applicable, are "killer features" for specific, important archetypes:
    *   Read/Write filtering is the key to unlocking value for CMS-like apps.
    *   Static Content Serving is the key to white-labeling classic enterprise apps.
    *   Auth Adaptation and Native Tenancy Orchestration are the key to enhancing modern, capable apps like Metabase.

This synthesized list provides a powerful, data-driven view of our feature set, allowing for informed decisions about the MVP scope and long-term roadmap.
