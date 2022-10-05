use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};

mod router;

// let the Ctrl+C signal gracefully shutdown the server
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    // init router
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(router::route))
    });

    // init server on http://127.0.0.1:2080
    let addr = SocketAddr::from(([127, 0, 0, 1], 2080));
    let server = hyper::Server::bind(&addr).serve(make_svc);

    // start server
    if let Err(e) = server.with_graceful_shutdown(shutdown_signal()).await {
        eprintln!("server error: {}", e);
    }
}
