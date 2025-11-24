# NAAAS Candidate Investigation Summary

This document summarizes the findings from the in-depth analysis of 10 diverse open-source applications, categorizing them by how NAAAS can best provide value as an "Intelligent Infrastructure Wrapper."

---
## Core Strategic Insight

The investigation confirms that the core value of NAAAS is in providing **Infrastructure-Level "Enterprise Theater"** by wrapping existing monolithic applications. Our role is to provide powerful, flexible infrastructure services at the edge, rather than solving application-specific business logic problems.

The analysis revealed distinct archetypes, each highlighting different strengths of NAAAS:

---
## 1. The "Content-Hackable" Monolith Archetype

*   **Examples:** Ghost, WordPress, Directus.
*   **Peculiarity:** These applications often have rich APIs with filtering mechanisms (tags, categories, custom fields) for content, but may lack robust user isolation or overall multi-tenancy design.
*   **NAAAS Opportunity (Intelligent Wrapper):** Our shim's **Read Filtering** and **Write Stamping** features are highly impactful here. They allow NAAAS to leverage the application's own API to create "soft" multi-tenancy for content with minimal or zero code changes to the monolith.
    *   **Ghost/WordPress:** "Tenant-as-a-Tag" for content is viable.
    *   **Directus:** Direct API filter injection is highly viable.
*   **User Isolation:** Typically requires a documented, minimal database modification (e.g., `naaas_tenant_id` column) for full isolation.

---
## 2. The "Fortress" Monolith Archetype

*   **Examples:** Redmine, SuiteCRM, osTicket, Gitea.
*   **Peculiarity:** These applications have rigid data models that are fundamentally single-tenant or whose internal multi-tenancy (e.g., SuiteCRM's Security Groups) is too complex or risky for an edge wrapper to manipulate. They often have global user namespaces.
*   **NAAAS Opportunity (Multi-Instance Manager):** For these applications, the "Intelligent Wrapper" (data filtering/stamping) is not feasible or desirable. Instead, NAAAS's value is in making the **"Separate Instances" model operationally trivial and economically viable.**
    *   NAAAS simplifies the deployment and management nightmare of running many separate instances (each with its own database) which is the *only* safe and supported way to multi-tenant these applications.

---
## 3. The "API-First & Decoupled" App Archetype

*   **Example:** Invoice Ninja (v5).
*   **Peculiarity:** Modern applications with great internal data separation, but whose developers may have explicitly removed single-instance multi-tenancy for self-hosters to drive users to their cloud offering.
*   **NAAAS Opportunity:** Similar to the "Fortress" archetype. We provide the exact "multi-instance" architecture the developers recommend, but with the operational simplicity of a single command (`naaas-ctl deploy`). This is a powerful and very clean value proposition.

---
## 4. The "Complex But Capable" App Archetype

*   **Examples:** Metabase, Chatwoot.
*   **Peculiarity:** These applications *already have* powerful, often complex, built-in multi-tenancy features.
*   **NAAAS Opportunity (Orchestrator & Enhancer):** Our role shifts from *creating* tenancy to **orchestrating and enhancing** it.
    *   **Simplifying Complexity:** We use our declarative `app_config` to automate the complex, UI-driven setup of their internal multi-tenancy features (e.g., Metabase's data sandboxing).
    *   **Democratizing Paid Features:** We provide capabilities like white-labeling (via Static Content Serving for Metabase) and advanced SSO (via our Auth Adapter for Metabase) that are often paywalled in the application's enterprise tiers.

---
## Cross-Cutting & Universal NAAAS Feature Value

Regardless of archetype, the following NAAAS features provide consistent, high value:

*   **Branding (Opaque Config & Static Content Serving):** Universal need across all categories for custom logos, colors, and themes.
*   **Performance (Caching):** Essential for all web applications, especially heavy ones like e-commerce (PrestaShop) or BI tools (Metabase).
*   **Operational Excellence (TLS, Rate Limiting, Logging):** Foundational requirements for any production deployment, simplifying management significantly.
*   **Auth Adaptation:** Valuable for unifying disparate authentication schemes or democratizing paid SSO features (e.g., Metabase).
*   **Networking Challenges:** Gitea (SSH) and Chatwoot (WebSockets) highlight that to be a truly generic platform, NAAAS will eventually need to proxy more than just standard HTTP traffic.

---
## Conclusion of Investigation

This deep dive confirms the broad applicability and value proposition of NAAAS across a diverse range of open-source monolithic applications. It clarifies that NAAAS operates in distinct modes for different archetypes:
1.  **Intelligent Wrapper:** For Content-Hackable apps, providing smart API interventions.
2.  **Multi-Instance Manager:** For Fortress/API-First apps, simplifying the deployment of separate instances.
3.  **Orchestrator/Enhancer:** For Complex/Capable apps, streamlining configuration and democratizing features.

This comprehensive understanding is critical for guiding the development of NAAAS, ensuring its features are both powerful and widely useful.
