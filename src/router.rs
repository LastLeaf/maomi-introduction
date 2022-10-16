use std::fs;
use hyper::{Body, Request, Response, Method, StatusCode};

use maomi_introduction::server_side_rendering;

pub(crate) async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    match req.method() {
        &Method::GET => {
            let req_path = req.uri().path();
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
        },
        _ => {}
    };
    // return 
    Response::builder().status(StatusCode::NOT_FOUND).body(Body::from(""))
}
