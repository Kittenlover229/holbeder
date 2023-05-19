use std::error::Error;

use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv()?;

    let port: u16 = dotenv::var("PORT")?.parse()?;
    let hostname: String = dotenv::var("HOSTNAME")?;

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind((hostname, port))?
        .run()
        .await?;

    Ok(())
}
