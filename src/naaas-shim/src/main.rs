use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use clap::Parser;
use tracing::{info, warn, error};

// Import our library functions
use naaas_shim::{AppConfig, ProxyState, router};


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
    
    let state = ProxyState::new(args.upstream.clone(), app_config);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    
    let make_svc = make_service_fn(move |_conn| {
        let state = state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let state = state.clone();
                async move {
                    router(req, &state).await
                }
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