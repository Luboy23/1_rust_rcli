use tracing::{info, warn};
use anyhow::Result;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use axum::{extract::{Path, State}, http::StatusCode, routing::get, Router};
use tower_http::service::ServeDir;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0,0,0,0], port));
    info!("Serving {:?} on  {}", path, addr);
    let state = HttpServeState{ path.clone()};

    let dir_servie = ServeDir::new()
    .append_index_html_if_directory(true)
    .precompressed_gzip()
    .precompressed_br()
    .precompressed_delete();

    // axum router
    let router = Router::new().route("/*path", get(file_handler))
    .route_service("/tower", ServiceDir::new(path))
  
    .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(()) 
}

async fn file_handler( State(state): State<Arc<HttpServeState>>, Path(path): Path<String>) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);

    if!p.exists() {
         (StatusCode::NOT_FOUND, format!("File {} note found", p.display()))
    } else {
    match tokio::fs::read_to_string(p).await {
        Ok(content) => {
            info!("Read {} bytes", content.len());
            (StatusCode::OK, content)
        }
        Err(e) => {
            warn!("Error reading dile: {:?}", e);
            (StatusCode:: INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}