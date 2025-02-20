use crate::common::Anubis;
use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use std::sync::Mutex;
use tera::Context;

pub trait AnubisServer {
    fn serve(
        self,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}

impl AnubisServer for Anubis {
    async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(Mutex::new(self));

        let app = Router::new()
            .route("/", get(home_page))
            .route("/get/graph", get(graph))
            .route("/{*Page}", get(page_endpoint))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();

        Ok(())
    }
}

async fn page_endpoint(
    State(state): State<Arc<Mutex<Anubis>>>,
    extract::Path(page_name): extract::Path<String>,
) -> impl IntoResponse {
    let state_access = state.lock(); //obtain access over the AnubisDatabase connection
    if state_access.is_ok() {
        let anubis = state_access.unwrap();
        if let Some(context) = anubis.database.get_context(&page_name) {
            if let Ok(rendered_page) = anubis.tera.render("page.html", &context) {
                return Html(rendered_page).into_response();
            }
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unable to access db".to_string(),
    )
        .into_response()
}

async fn graph(State(state): State<Arc<Mutex<Anubis>>>) -> impl IntoResponse {
    let state_access = state.lock();
    if state_access.is_ok() {
        let anubis = state_access.unwrap();
        if let Ok(graph) = serde_json::to_string(&anubis.database.graph_db) {
            return Json(graph).into_response();
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unable to access graph db".to_string(),
    )
        .into_response()
}

async fn home_page(State(state): State<Arc<Mutex<Anubis>>>) -> impl IntoResponse {
    let state_access = state.lock();
    if state_access.is_ok() {
        let anubis = state_access.unwrap();
        if let Ok(rendered_page) = anubis.tera.render("index.html", &Context::new()) {
            return Html(rendered_page).into_response();
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unable to access graph db".to_string(),
    )
        .into_response()
}
