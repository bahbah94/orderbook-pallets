mod routes;
mod ws;
mod http_client;

use crate::env::Config;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::tokio::TokioIo;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, util::SubscriberInitExt};

/// Start HTTP (no TLS) with WS upgrade support.
pub fn start(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        // idempotent logger init
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        let client = http_client::make_client();

        let addr = format!("0.0.0.0:{}", cfg.indexer_port);
        let listener = TcpListener::bind(&addr).await?;
        info!("listening on http://{}", addr);

        loop {
            let (tcp, peer) = listener.accept().await?;
            let client = client.clone();
            let cfg = cfg.clone();

            tokio::spawn(async move {
                let io = TokioIo::new(tcp); // <— makes tokio stream implement hyper Read/Write
                let svc = service_fn(move |req| routes::route(req, client.clone(), cfg.clone()));

                if let Err(e) = http1::Builder::new().serve_connection(io, svc).await {
                    error!("serve_connection error from {peer:?}: {e}");
                }
            });
        }
    })
}
