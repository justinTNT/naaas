use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing::{info, error};

// Import our modularized components
use naaas_server::{TenantStore, handle_deploy, handle_list_tenants, handle_delete_tenant, handle_health};

async fn router(
    req: Request<Body>,
    store: TenantStore,
) -> Result<Response<Body>, Infallible> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::POST, "/deploy") => {
            handle_deploy(req.into_body(), store).await
        }
        (&Method::GET, "/tenants") => {
            handle_list_tenants(store).await
        }
        (&Method::DELETE, path) if path.starts_with("/tenants/") => {
            let tenant_id = path.strip_prefix("/tenants/").unwrap().to_string();
            handle_delete_tenant(tenant_id, store).await
        }
        (&Method::GET, "/health") => {
            handle_health().await
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    };
    
    match response {
        Ok(resp) => Ok(resp),
        Err(err) => {
            error!("Request handler error: {}", err);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let store: TenantStore = Arc::new(Mutex::new(HashMap::new()));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    let make_svc = make_service_fn(move |_conn| {
        let store = store.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                router(req, store.clone())
            }))
        }
    });
    
    let server = Server::bind(&addr).serve(make_svc);
    
    info!("NAAAS Server starting on http://{}", addr);
    info!("Endpoints:");
    info!("  POST /deploy      - Deploy a new tenant");
    info!("  GET  /tenants     - List all tenants");
    info!("  DELETE /tenants/{{id}} - Delete a tenant");
    info!("  GET  /health      - Health check");
    
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
    
    Ok(())
}