use crate::env::Config;
use crate::server::ws;
use bytes::Bytes;
use fastwebsockets::upgrade;
use http_body_util::Full;
use hyper::{Request, Response, StatusCode, body::Incoming};
use std::sync::Arc;

pub async fn route(
    req: Request<Incoming>,
    http_client: Arc<reqwest::Client>,
    _cfg: Config,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    let path = req.uri().path();

    match (req.method().as_str(), path) {
        ("GET", "/health") => text(StatusCode::OK, "ok"),

        ("GET", "/api/hello") => {
            let body = match http_client
                .get("https://worldtimeapi.org/api/timezone/Etc/UTC")
                .send()
                .await
            {
                Ok(resp) => resp.text().await.unwrap_or_else(|_| "{\"status\":\"ok\"}".into()),
                Err(_) => "{\"status\":\"ok\"}".into(),
            };
            json(StatusCode::OK, body)
        }

        ("GET", "/ws") => {
            // fastwebsockets 0.9: upgrade takes the request (by value) and no second arg
            let (response, fut) = match upgrade::upgrade(req) {
                Ok(ok) => ok,
                Err(err) => return text(StatusCode::BAD_REQUEST, &format!("upgrade failed: {err}")),
            };

            tokio::spawn(async move {
                match fut.await {
                    Ok(ws) => ws::echo_loop(ws).await,
                    Err(e) => tracing::error!("websocket upgrade future failed: {e}"),
                }
            });

            Ok(response.map(|_| Full::new(Bytes::new())))
        }

        _ => text(StatusCode::NOT_FOUND, "not found"),
    }
}

fn text(code: StatusCode, s: &str) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    Response::builder()
        .status(code)
        .header("content-type", "text/plain; charset=utf-8")
        .body(Full::new(Bytes::from(s.to_owned())))
}

fn json(code: StatusCode, s: String) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    Response::builder()
        .status(code)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(s)))
}
