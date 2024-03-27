mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};

use api::user_api::{delete, index, show, store, update};
use repository::mongo_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    return HttpServer::new(move| | {
        App::new()
            .app_data(db_data.clone())
            .service(index)
            .service(store)
            .service(show)
            .service(update)
            .service(delete)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await;
}
