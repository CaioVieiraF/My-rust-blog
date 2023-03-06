pub mod threads;
pub mod schema;
pub mod models;

use actix_web::{HttpServer, App, web};
use diesel::{r2d2::Pool, SqliteConnection, r2d2::ConnectionManager};
use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
struct AppState {
    database: Pool<ConnectionManager<SqliteConnection>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let manager = ConnectionManager::<SqliteConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    let pool = Pool::builder().build(manager).unwrap();
    let state = web::Data::new( AppState {
        database: pool,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(threads::config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
