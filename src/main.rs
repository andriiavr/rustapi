use axum::{Router, routing::get};

mod handlers;
use handlers::{calculate, get_posts, new_post};
// use sqlx::PgPool;

// pub struct AppState {
//     db: PgPool
// }

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/posts", get(get_posts).post(new_post))
        .route("/calc", get(calculate));

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}


