pub mod app;
pub mod http;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use app::app_setup::Dirtybase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = Dirtybase::new().await.unwrap();

    app.db_setup().await;

    if app.schema_manger().has_table("students").await {
        println!("table exist");
    } else {
        println!("table does not exist");
    }

    let x = web::Data::new(app);

    HttpServer::new(move || {
        App::new()
            .app_data(x.clone())
            .service(hello)
            .service(serve_users)
    })
    .bind(("127.0.0.1", 8080))?
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
