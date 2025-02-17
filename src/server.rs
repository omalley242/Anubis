use crate::db::{get_page, open_db};
use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rusqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let db = open_db()?;
    let state = Arc::new(Mutex::new(db));

    let app = Router::new()
        .route("/", get(home_page))
        .route("/{*Page}", get(page_endpoint))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn page_endpoint(
    State(state): State<Arc<Mutex<Connection>>>,
    extract::Path(page_name): extract::Path<String>,
) -> impl IntoResponse {
    println!("{}", page_name);
    let state_access = state.lock(); //obtain access over the database connection
    if state_access.is_ok() {
        let db = state_access.unwrap();
        let page_result = get_page(&db, &page_name);

        if page_result.is_ok() {
            let page = page_result.unwrap();
            Html(page).into_response()
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to access page".to_string(),
            )
                .into_response()
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to access db".to_string(),
        )
            .into_response()
    }
}

async fn home_page() -> impl IntoResponse {
    // Generate File list
}
