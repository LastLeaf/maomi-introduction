use hyper::{Body, Method, Request, Response, StatusCode};
use std::fs;

#[cfg(feature = "server-side-rendering")]
use maomi_introduction::server_side_rendering;

pub(crate) async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    let ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|x| x.to_str().ok())
        .unwrap_or("(unknown)");
    log::info!("visitor {}: {:?} {}", ip, req.method(), req.uri());
    let path_prefix = maomi_introduction::config::path_prefix();
    match req.method() {
        &Method::GET => {
            if let Some(req_path) = req.uri().path().strip_prefix(path_prefix) {
                let req_path = if req_path != "" { req_path } else { "/" };

                // serve static files
                if req_path.starts_with("/pkg/") || req_path.starts_with("/res/") {
                    if let Ok(content) = fs::read(req_path.trim_start_matches("/")) {
                        let mime_type = mime_guess::from_path(req_path).first();
                        let mime_type = mime_type
                            .as_ref()
                            .map(|x| x.essence_str())
                            .unwrap_or("application/octet-stream");
                        return Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", mime_type)
                            .body(Body::from(content));
                    } else {
                        return Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .header("Content-Type", "text/plain")
                            .body(Body::from(""));
                    }
                }

                // serve components
                let query = req.uri().query().unwrap_or_default();
                #[cfg(feature = "server-side-rendering")]
                {
                    match server_side_rendering(req_path, query).await {
                        Ok(html) => {
                            return Response::builder()
                                .status(StatusCode::OK)
                                .header("Content-Type", "text/html")
                                .body(Body::from(html));
                        }
                        Err(status_code) => {
                            return Response::builder()
                                .status(status_code)
                                .header("Content-Type", "text/plain")
                                .body(Body::from(""));
                        }
                    }
                }
                #[cfg(not(feature = "server-side-rendering"))]
                {
                    let html = format!(
                        include_str!("component_html.tmpl"),
                        path_prefix = path_prefix,
                        title = "",
                        body = "",
                        req_path = req_path,
                        data = query,
                    );
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/html")
                        .body(Body::from(html));
                }
            }
        }
        _ => {}
    };

    // return 403 for unknown requests
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::from(""))
}
