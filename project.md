# Project NAAAS: A Thin Infrastructure Wrapper for Monolithic Applications

## 1. Our Guiding Philosophy: Do What Is Natural at This Layer

Our goal is to build a **tool**, not a product. This is a technical exploration to answer the question: "What useful, infrastructure-level features can we provide by wrapping a monolithic application in a Rust unikernel?"

Our core principle is **"Enterprise Theater"**: we provide the *appearance* and *operational benefits* of an enterprise-grade deployment (per-tenant domains, TLS, monitoring) without the immense complexity of re-engineering the application itself.

We will resist the urge to solve application-level problems. Our role is to be a **thin infrastructure wrapper** and to only do what comes naturally at the network edge.

## 2. The Core Problem & Our Role

Developers often build useful, monolithic applications that are not designed for multi-tenancy. When they need to serve multiple customers, they face a "Great Divide" of complex infrastructure work.

NAAAS acts as a **"Private VIP Entrance"** to this monolith:
*   We provide a secure, unique door for each tenant (`https://tenant-a.com` with TLS).
*   We put a bouncer at the door (rate-limiting).
*   We put a camera at the door (logging and monitoring).
*   We can hang a sign with the tenant's name above the door (via a `/config` endpoint).

**Crucially, what happens inside the application is not our game.** Data separation is the user's responsibility. They can achieve it by pointing different NAAAS shims to different application instances or databases. Our job is to make the "entrance" so efficient and easy to deploy that managing multiple instances becomes trivial.

## 3. The Solution: The "Thin Wrapper" Shim

We wrap existing applications with a "Tenant Shim": a lightweight, secure, and isolated unikernel deployed per-tenant. The shim's responsibilities are strictly limited to the infrastructure layer:

*   **TLS Termination:** Uses client-provided certificates for the tenant's domain.
*   **Transparent Proxying:** Forwards all traffic to the upstream application without inspecting or modifying application-specific content (like `Authorization` headers or response bodies).
*   **Rate Limiting:** Enforces simple, per-tenant request limits.
*   **Lightweight Logging:** Emits structured access logs to `stdout`.
*   **Opaque Configuration Serving:** Exposes a `/config` endpoint to serve a user-provided, opaque JSON blob for branding generic frontends.

**Explicitly Out of Scope:** Data filtering, application-level authentication, API parsing, and business logic of any kind.

## 4. Product & Adoption Strategy: Fun Mode vs. Safe Mode

*   **"Fun Mode" (The Appliance):** Pre-built packages (e.g., an AWS AMI for Ghost) for quick evaluation and non-critical projects. This is our "hook" to demonstrate value.
*   **"Safe Mode" (The Artisan Tool):** The only supported path for production. Users provision their own infrastructure and **compile the unikernel shim from our open-source code** using our reproducible build process. Our adoption page will guide serious users directly to this path.

## 5. Scope Boundary

NAAAS is a purely operational and technical tool for exploring unikernel capabilities. It is not a sales, billing, or CRM product. Its workflow begins after a new "tenant" has been decided upon.