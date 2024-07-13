use clap::Parser;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
};
use std::convert::Infallible;
use std::net::SocketAddr;

mod router;

/// The server for maomi introduction website
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CmdArgs {
    /// The address to listen
    #[arg(short, long, required(true))]
    pub(crate) addr: String,
}

pub(crate) fn cmd_args() -> &'static CmdArgs {
    thread_local! {
        static CMD_ARGS: &'static CmdArgs = Box::leak(Box::new(CmdArgs::parse()));
    }
    CMD_ARGS.with(|x| *x)
}

// let the Ctrl+C signal gracefully shutdown the server
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    // enable log
    env_logger::init();

    // init router
    let make_svc = make_service_fn(|conn: &AddrStream| {
        log::info!("Connected from {:?}", conn.remote_addr());
        async { Ok::<_, Infallible>(service_fn(router::route)) }
    });

    // init server
    let addr_str = cmd_args().addr.as_str();
    let addr: SocketAddr = addr_str.parse().unwrap();
    let server = hyper::Server::bind(&addr).serve(make_svc);

    // start server
    if let Err(e) = server.with_graceful_shutdown(shutdown_signal()).await {
        eprintln!("server error: {}", e);
    }
}
