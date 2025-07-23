mod config;
mod database;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod schema;
mod utilities;

use actix_web::{App, web, HttpServer};
use config::Config;
use database::DatabasePool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();
  env_logger::init();
  let config = Config::load().unwrap();  // ✅ Config loads successfully
  let pool = DatabasePool::new(&config.database.url);

  println!("Server starting at {}:{}", config.server.host, config.server.port);

  // Clone config for use in the closure
  let config_for_app = config.clone();

  // Start Actix web server
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .app_data(web::Data::new(config_for_app.clone()))
      .configure(routes::configure)
  })
  .bind((config.server.host, config.server.port))?
  .run()
  .await
}