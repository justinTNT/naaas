# Hermit "Hello, Hyper!" Proof-of-Concept Plan

This document outlines the step-by-step process required to build and deploy a minimal "Hello, Hyper!" web server as a Hermit unikernel. This Proof-of-Concept (PoC) serves to validate our chosen technology stack (Hermit with `hermit-os/tokio` and `hyper`) before proceeding with the main NAAAS development.

---
**Goal:** To prove that a `hyper`-based web server can successfully run on Hermit using the `hermit-os/tokio` fork.

**Success Criterion:** Successfully receive an HTTP "Hello, Hermit!" response from a `curl` request made from the host machine to the `hyper` server running inside the Hermit unikernel.

**Estimated Time:** 1-2 hours (assuming a clean Linux environment with `rustup` already installed).

---

## Step 1: Hermit Development Environment Setup

*   **1.1. Install `rustup` (if not already present):**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
*   **1.2. Add Hermit Rust Target:**
    ```bash
    rustup target add x86_64-unknown-hermit
    rustup component add rust-std --target x86_64-unknown-hermit
    ```
*   **1.3. Install `uhyve` (Hermit's Unikernel Hypervisor):**
    *   `uhyve` is required to run Hermit applications. Its installation method can vary (e.g., `apt install uhyve` or building from source). The user executing this plan should consult the official HermitOS documentation ([hermit-os.org](https://hermit-os.org/)) for the most current and specific installation instructions to ensure `uhyve` is correctly installed and available in the system's PATH.
    *   **Verification:** `uhyve --version`

---

## Step 2: Create the "Hello, Hyper!" Project

*   **2.1. Create Project Directory:**
    ```bash
    cargo new hermit-hyper-poc
    cd hermit-hyper-poc
    ```
*   **2.2. Configure `Cargo.toml`:**
    *   Add `hyper` and the Hermit-compatible `tokio` as dependencies.
    *   Add build configuration for the Hermit target and `uhyve` runner.
    ```toml
    # hermit-hyper-poc/Cargo.toml
    [package]
    name = "hermit-hyper-poc"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    hyper = { version = "0.14", features = ["full"] } # Using 0.14 for broader compatibility in PoC
    tokio = { git = "https://github.com/hermit-os/tokio.git", features = ["full"] }

    [build]
    target = "x86_64-unknown-hermit"

    [target.'cfg(target_os = "hermit")']
    runner = "uhyve"
    ```
*   **2.3. Implement "Hello, Hyper!" Server (`src/main.rs`):**
    *   Write a minimal `hyper`-based HTTP server that listens on port `8080` and responds with "Hello, Hermit!"
    ```rust
    // src/main.rs
    use hyper::service::{make_service_fn, service_fn};
    use hyper::{Body, Request, Response, Server};
    use tokio::net::TcpListener;
    use std::convert::Infallible;
    use std::net::SocketAddr;

    async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new("Hello, Hermit!".into()))
    }

    #[tokio::main]
    async fn main() {
        // Hardcode the address for PoC simplicity. In actual shim, this would be configurable.
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(handle_request))
        });

        let server = Server::bind(&addr).serve(make_svc);

        println!("Listening on http://{}", addr);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
    ```

---

## Step 3: Build and Run the Unikernel

*   **3.1. Build the Project:**
    ```bash
    cargo build --release --target x86_64-unknown-hermit
    ```
*   **3.2. Run the Unikernel:**
    *   The `Cargo.toml` runner configuration with `uhyve` should allow for a simplified `cargo run`. This will launch `uhyve` and the unikernel.
    *   Ensure `uhyve`'s networking is configured to allow host access to the unikernel's port 8080.
    ```bash
    cargo run --release --target x86_64-unknown-hermit
    ```
    *(Alternative if `cargo run` fails: Manually launch with `uhyve`. Consult `uhyve` documentation for correct networking flags, typically involving port forwarding: `uhyve -p 8080:8080 target/x86_64-unknown-hermit/release/hermit-hyper-poc`)*

---

## Step 4: Verify the PoC

*   **4.1. From the Host Machine:**
    ```bash
    curl http://127.0.0.1:8080
    ```
*   **4.2. Expected Output:**
    ```
    Hello, Hermit!
    ```
---
