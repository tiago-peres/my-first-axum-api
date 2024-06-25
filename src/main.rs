use axum::{
    routing::{get},
    Router,
    Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use std::net::SocketAddr;
use std::sync::Arc;
use dotenv::dotenv;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let (client, connection) = tokio_postgres::connect(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        NoTls,
    )
    .await
    .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client = Arc::new(client);

    let app = Router::new()
        .route("/users", get(get_users).post(add_user))
        .layer(Extension(client))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_users(Extension(client): Extension<Arc<tokio_postgres::Client>>) -> Json<Vec<User>> {
    let rows = client
        .query("SELECT * FROM users", &[])
        .await
        .expect("Failed to execute query");
    let users = rows
        .into_iter()
        .map(|row| User {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect();
    Json(users)
}

#[derive(Deserialize)]
struct AddUser {
    name: String,
}

async fn add_user(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    Json(payload): Json<AddUser>,
) -> Json<User> {
    let row = client
        .query_one("INSERT INTO users (name) VALUES ($1) RETURNING id, name", &[&payload.name])
        .await
        .expect("Failed to execute query");
    let user = User {
        id: row.get("id"),
        name: row.get("name"),
    };
    Json(user)
}
