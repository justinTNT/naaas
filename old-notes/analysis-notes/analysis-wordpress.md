# WordPress (PHP) Analysis

## 1. Architecture & Tech Stack

*   **Core:** PHP application.
*   **Database:** Primarily MySQL/MariaDB.
*   **Frontend:** Server-side rendered HTML with JavaScript (jQuery common), CSS. Modern versions include the REST API and block editor for more dynamic frontends, but the core remains server-rendered.
*   **Monolithic Nature:** Highly monolithic. Core application, themes, and plugins all interact deeply within a single runtime.
*   **Plugin Ecosystem:** Massive, highly extensible via hooks and filters. This is both its strength and its source of "messiness."

## 2. Authentication Model

*   **Primary:** Cookie-based session authentication for user logins (after username/password via `wp-login.php`).
*   **REST API:** Supports cookie-based auth (for logged-in users), basic authentication (requires a plugin), and OAuth 1.0a (requires a plugin). The most common for external apps is application passwords (a form of bearer token) or OAuth.
*   **API Keys:** No native concept of a general "API Key" for third-party services without a plugin. Plugins often introduce their own API key mechanisms.

## 3. Data Isolation Analysis

*   **Native Multi-tenancy (WordPress Multisite):** WordPress has a built-in feature called "Multisite" designed to host multiple independent "sites" (blogs) from a single WordPress installation.
    *   **Mechanism:** It uses a single database, but with separate tables for each site (e.g., `wp_posts`, `wp_users` for the main site, `wp_2_posts`, `wp_2_users` for site ID 2). Super-admins manage all sites.
    *   **Drawbacks:** It's complex to set up, resource-heavy if many sites are active, and plugins often don't fully support it, leading to issues. Cross-site data leakage *can* occur if plugins are not carefully coded. It's multi-tenancy *by design*, but still suffers from performance and configuration complexity.
*   **Without Multisite:** A standard WordPress installation is strictly single-tenant. All posts, users, comments, etc., belong to that one site.
*   **Read Filtering Opportunity:**
    *   **WordPress REST API:** The REST API is robust and supports filtering posts by `author`, `category`, `tag`, and custom taxonomies. We could leverage a "Tenant-as-a-Tag" or "Tenant-as-a-Custom-Taxonomy" approach similar to Ghost, where each tenant's content is identified by a unique, hidden tag/taxonomy term.
    *   **User Data:** User data is not natively taggable. For full user isolation (multiple users with the same email across tenants), a database modification would be ideal.
*   **Write Stamping Opportunity:**
    *   The REST API allows creation of posts and associating them with categories/tags/custom taxonomies. The shim could inject the `tenant_id` as a tag or custom taxonomy term.

## 4. Configuration & Branding Pain Points

*   **Configuration:** `wp-config.php` is the primary configuration file (database details, security keys, debugging settings). Many settings are stored in the database. Plugins often have their own config files or database tables.
*   **Branding:** Themes control branding. Customization is typically done via the WordPress Customizer, theme options, or directly editing theme files. This makes per-tenant branding extremely complex without Multisite, as a theme applies globally.
*   **NAAAS Opportunity:** Our `/config` endpoint for serving `app_config` is a perfect fit here. A generic frontend (or a modified theme) could fetch branding details (colors, logo, title) from the shim and apply them without touching `wp-config.php` or the theme files.

## 5. NAAAS Opportunity Scorecard (Initial Assessment)

*   **TLS Termination:** High impact. WordPress usually relies on Nginx/Apache for this.
*   **Transparent Proxying:** High impact. Necessary to put a shim in front.
*   **Rate Limiting:** High impact. Protects the PHP backend from abuse.
*   **Lightweight Logging:** High impact. Standardizes access logs.
*   **Opaque Config Serving:** High impact. Perfect for out-of-band branding configuration.
*   **Static Content Serving:** High impact. Can offload WordPress's static asset serving.
*   **Caching:** High impact. WordPress is notoriously slow without aggressive caching.
*   **Read Filtering (Content):** High impact. Leverages REST API tags/taxonomies.
*   **Write Stamping (Content):** High impact. Leverages REST API tags/taxonomies.
*   **Auth Adaptation:** Medium impact. WordPress has many auth plugins; our shim could unify different schemes.

## 6. "Minimal Modification" Hypothesis

*   **Content Isolation:** Potentially **zero code changes** for content (posts, pages, custom post types) if we leverage the WordPress REST API and assign a unique, hidden tag or custom taxonomy term to each tenant's content. The shim would inject this on write and filter on read.
*   **User Isolation:** Requires a database modification similar to Ghost. The `wp_users` table needs a `naaas_tenant_id` column to allow multiple users with the same email/username across different tenants. Alternatively, a plugin could manage this if it exposes an API for user management with tenant scoping. This would need further investigation.
*   **Branding/Configuration:** Zero code changes needed. Handled by our `/config` endpoint and potentially Static Content Serving.

**Conclusion:** WordPress is an excellent candidate. It presents a rich environment to test all our "Intelligent Wrapper" features, especially content filtering and branding for a truly monolithic, widely-used application. The complexity of its native multi-site feature makes our simpler, edge-based approach very appealing.
