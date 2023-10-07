use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
}

#[get("/ping")]
async fn ping(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        "SELECT name, email FROM users WHERE email = 'admin@admin.com'"
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    HttpResponse::Ok().json(query_result)
}

struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(ping)
            .wrap(cors)
    })
    .bind(("127.0.0.1", 18200))?
    .run()
    .await
}
