use axum::{
    Router,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse, Json},
    routing::{get, post},
};
use dx_icon::{engine::IconSearchEngine, index::IconIndex};
use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    engine: Arc<IconSearchEngine>,
    data_dir: PathBuf,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default)]
    pack: Option<String>,
}

fn default_limit() -> usize {
    1000
}

#[derive(Serialize)]
struct SearchResult {
    name: String,
    pack: String,
    score: f32,
}

#[derive(Serialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
    count: usize,
    query: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = match load_state() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load icon index: {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/search", get(search_handler))
        .route("/api/svg/{pack}/{name}", get(svg_handler))
        .route("/api/download", post(download_handler))
        .nest_service("/static", ServeDir::new("web/static"))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Icon Search Web running at http://localhost:3000");
    println!("305,612+ icons ready to search");

    axum::serve(listener, app).await.unwrap();
}

fn load_state() -> anyhow::Result<AppState> {
    let mut possible_paths = vec![
        PathBuf::from("index"),
        PathBuf::from("crates/media/icon/index"),
    ];

    if let Ok(env_path) = std::env::var("DX_ICON_INDEX") {
        possible_paths.insert(0, PathBuf::from(env_path));
    }

    let index_dir = possible_paths
        .iter()
        .find(|p| p.exists())
        .ok_or_else(|| anyhow::anyhow!("Index not found"))?;

    let index = IconIndex::load_mmap(index_dir)?;
    let engine = IconSearchEngine::from_index(index)?;

    let mut data_paths = vec![
        PathBuf::from("data"),
        PathBuf::from("crates/media/icon/data"),
    ];

    if let Ok(env_path) = std::env::var("DX_ICON_DATA") {
        data_paths.insert(0, PathBuf::from(env_path));
    }

    let data_dir = data_paths
        .iter()
        .find(|p| p.exists())
        .ok_or_else(|| anyhow::anyhow!("Data directory not found"))?
        .clone();

    Ok(AppState {
        engine: Arc::new(engine),
        data_dir,
    })
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../../web/index.html"))
}

async fn search_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Json<SearchResponse> {
    let mut results = state.engine.search(&params.q, params.limit * 10);

    if let Some(pack) = &params.pack {
        results.retain(|r| &r.icon.pack == pack);
        results.truncate(params.limit);
    } else {
        results.truncate(params.limit);
    }

    let search_results: Vec<SearchResult> = results
        .into_iter()
        .map(|r| SearchResult {
            name: r.icon.name,
            pack: r.icon.pack,
            score: r.score,
        })
        .collect();

    let count = search_results.len();

    Json(SearchResponse {
        results: search_results,
        count,
        query: params.q,
    })
}

async fn svg_handler(
    State(state): State<AppState>,
    Path((pack, name)): Path<(String, String)>,
) -> impl IntoResponse {
    match generate_svg(&state.data_dir, &name, &pack) {
        Ok(svg) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "image/svg+xml")],
            svg,
        )
            .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Icon not found").into_response(),
    }
}

#[derive(Deserialize)]
struct DownloadRequest {
    icons: Vec<IconRef>,
}

#[derive(Deserialize)]
struct IconRef {
    name: String,
    pack: String,
}

async fn download_handler(
    State(state): State<AppState>,
    Json(req): Json<DownloadRequest>,
) -> impl IntoResponse {
    if req.icons.is_empty() {
        return (StatusCode::BAD_REQUEST, "No icons specified").into_response();
    }

    if req.icons.len() == 1 {
        let icon = &req.icons[0];
        match generate_svg(&state.data_dir, &icon.name, &icon.pack) {
            Ok(svg) => {
                let filename = format!("attachment; filename=\"{}_{}.svg\"", icon.pack, icon.name);
                (
                    StatusCode::OK,
                    [
                        (header::CONTENT_TYPE, "image/svg+xml"),
                        (header::CONTENT_DISPOSITION, filename.as_str()),
                    ],
                    svg,
                )
                    .into_response()
            }
            Err(_) => (StatusCode::NOT_FOUND, "Icon not found").into_response(),
        }
    } else {
        (
            StatusCode::NOT_IMPLEMENTED,
            "Bulk download not yet implemented",
        )
            .into_response()
    }
}

fn generate_svg(data_dir: &PathBuf, name: &str, pack: &str) -> anyhow::Result<String> {
    let pack_file = data_dir.join(format!("{}.json", pack));
    let content = fs::read_to_string(&pack_file)?;
    let pack_data: serde_json::Value = serde_json::from_str(&content)?;

    let icon_data = pack_data["icons"]
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("Icon not found"))?;

    let body = icon_data["body"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Icon body not found"))?;

    let width = icon_data["width"]
        .as_f64()
        .or_else(|| pack_data["width"].as_f64())
        .unwrap_or(24.0);

    let height = icon_data["height"]
        .as_f64()
        .or_else(|| pack_data["height"].as_f64())
        .unwrap_or(24.0);

    Ok(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">{}</svg>"#,
        width, height, width, height, body
    ))
}
