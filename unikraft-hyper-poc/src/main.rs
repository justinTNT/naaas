use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Result};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello(_: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello, Hyper!")))
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}