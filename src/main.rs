use axum::{
    routing::{get, post, put, delete},
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

mod cornucopia;
use cornucopia::queries::users::{
    get_all_users, insert_user, get_user, delete_user as db_delete_user, update_user,
};

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
struct AddUser {
    name: String,
}

#[derive(Deserialize)]
struct EditUser {
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
        .route("/users/:id", get(get_single_user).put(edit_user).delete(delete_user))
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
    let mut stmt = get_all_users();
    let rows = stmt.bind(&*client).all().await.expect("Failed to execute query");
    let users = rows.iter().map(|row| User {
        id: row.id,
        name: row.name.clone(),
    }).collect();
    Json(users)
}

async fn add_user(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    Json(payload): Json<AddUser>,
) -> Json<User> {
    let mut stmt = insert_user();
    let row = stmt.bind(&*client, &payload.name).one().await.expect("Failed to execute query");
    let user = User {
        id: row.id,
        name: row.name.clone(),
    };
    Json(user)
}

async fn get_single_user(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Json<User> {
    let mut stmt = get_user();
    let row = stmt.bind(&*client, &id).one().await.expect("Failed to execute query");
    let user = User {
        id: row.id,
        name: row.name.clone(),
    };
    Json(user)
}

async fn delete_user(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Json<User> {
    let mut stmt = db_delete_user();
    let row = stmt.bind(&*client, &id).one().await.expect("Failed to execute query");
    let user = User {
        id: row.id,
        name: row.name.clone(),
    };
    Json(user)
}

async fn edit_user(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(payload): Json<EditUser>,
) -> Json<User> {
    let mut stmt = update_user();
    let row = stmt.bind(&*client, &payload.name, &id).one().await.expect("Failed to execute query");
    let user = User {
        id: row.id,
        name: row.name.clone(),
    };
    Json(user)
}
