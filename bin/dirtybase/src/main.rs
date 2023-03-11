pub mod app;
pub mod http;

use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use app::app_setup::Dirtybase;
use dotenv::dotenv;
use log::{error, info};
use pretty_env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    if let Err(e) = dotenv() {
        error!("could not load .env file: {:#}", e);
    }

    let db_connection = if let Ok(conn) = env::var("DTY_DATABASE") {
        conn
    } else {
        error!("error");
        "".to_owned()
    };

    let max_connection: u32 = if let Ok(max) = env::var("DTY_DATABASE_MAX_POOL_CONNECTION") {
        max.parse().unwrap_or(5)
    } else {
        5
    };

    let app = Dirtybase::new(&db_connection, max_connection)
        .await
        .unwrap();
    app.db_setup().await;

    let data = web::Data::new(app);
    let port: u16 = if let Ok(p) = env::var("DTY_WEB_PORT") {
        p.parse().unwrap_or(8080)
    } else {
        8080
    };

    info!("Server running on port: {}", port);
    info!("DB max pool: {}", db_connection);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(serve_users)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users")]
async fn serve_users(app: web::Data<Dirtybase>) -> impl Responder {
    let mut manager = app.schema_manger();
    let result = manager
        .table("_core_users", |query| {
            query.is_in("internal_id", vec![2, 1, 40]);
        })
        .fetch_all_as_json()
        .await;

    HttpResponse::Ok().json(result)
}
