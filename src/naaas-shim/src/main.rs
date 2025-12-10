use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server, StatusCode, Uri};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::SocketAddr;
use clap::Parser;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    logo_url: Option<String>,
    primary_color: Option<String>,
}

#[derive(Parser)]
#[command(name = "naaas-shim")]
#[command(about = "NAAAS Transparent Proxy Shim")]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value = "3000")]
    port: u16,
    
    /// Upstream URL to proxy to
    #[arg(short, long, default_value = "http://localhost:2368")]
    upstream: String,
    
    /// App configuration JSON
    #[arg(short, long)]
    config: Option<String>,
}

struct ProxyState {
    upstream_url: String,
    client: Client<hyper::client::HttpConnector>,
    app_config: Option<AppConfig>,
}

async fn handle_config(state: &ProxyState) -> Result<Response<Body>, anyhow::Error> {
    match &state.app_config {
        Some(config) => {
            let response = serde_json::to_string(config)?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(response))?)
        }
        None => {
            let default_config = AppConfig {
                name: "Default App".to_string(),
                logo_url: None,
                primary_color: Some("#007acc".to_string()),
            };
            let response = serde_json::to_string(&default_config)?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(response))?)
        }
    }
}

async fn proxy_request(
    mut req: Request<Body>,
    state: &ProxyState,
) -> Result<Response<Body>, anyhow::Error> {
    info!("Proxying {} {}", req.method(), req.uri().path());
    
    // Build the upstream URL
    let upstream_uri = format!(
        "{}{}{}",
        state.upstream_url.trim_end_matches('/'),
        req.uri().path(),
        req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default()
    );
    
    let uri: Uri = upstream_uri.parse()?;
    *req.uri_mut() = uri;
    
    // Remove hop-by-hop headers
    req.headers_mut().remove("host");
    req.headers_mut().remove("connection");
    req.headers_mut().remove("upgrade");
    req.headers_mut().remove("proxy-authorization");
    req.headers_mut().remove("proxy-authenticate");
    req.headers_mut().remove("te");
    req.headers_mut().remove("trailer");
    req.headers_mut().remove("transfer-encoding");
    
    // Forward the request to the upstream
    let response = state.client.request(req).await?;
    Ok(response)
}

async fn router(
    req: Request<Body>,
    state: &ProxyState,
) -> Result<Response<Body>, Infallible> {
    let result = match (req.method(), req.uri().path()) {
        (&Method::GET, "/config") => {
            handle_config(state).await
        }
        _ => {
            proxy_request(req, state).await
        }
    };
    
    match result {
        Ok(resp) => Ok(resp),
        Err(err) => {
            error!("Request handler error: {}", err);
            Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from("Bad Gateway"))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    
    // Parse app config if provided
    let app_config = if let Some(config_str) = &args.config {
        match serde_json::from_str::<AppConfig>(config_str) {
            Ok(config) => Some(config),
            Err(e) => {
                warn!("Failed to parse app config: {}. Using default.", e);
                None
            }
        }
    } else {
        None
    };
    
    let state = ProxyState {
        upstream_url: args.upstream.clone(),
        client: Client::new(),
        app_config,
    };
    
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    
    let make_svc = make_service_fn(move |_conn| {
        let state = state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                router(req, &state)
            }))
        }
    });
    
    let server = Server::bind(&addr).serve(make_svc);
    
    info!("NAAAS Shim starting on http://{}", addr);
    info!("Proxying to upstream: {}", args.upstream);
    info!("Endpoints:");
    info!("  GET /config  - Get app configuration");
    info!("  *           - Proxy all other requests to upstream");
    
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
    
    Ok(())
}