use axum::{
    routing::get,
    Router,
    http::{StatusCode}
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn internal_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/-1/error", get(internal_error))
        .route("/", get(hello_world));
    Ok(router.into())
}
