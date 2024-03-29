use std::env;
use std::sync::Arc;
use actix_files::Files as Fs;
use actix_web::{
    App, HttpServer, middleware, web,
};
use listenfd::ListenFd;
use morioka_service::sea_orm::Database;
mod common;
mod logic;
mod util;
use tokio::sync::RwLock as AsyncRwLock;
use crate::common::AppState;
#[actix_web::main]
async fn start(
) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    // get env vars
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let conn = Database::connect(&db_url).await.unwrap();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");
    let redis_box = crate::util::MoriokaRedis::new(&redis_url);
    let redis = redis_box.expect("get redis pool error!");
    let app_state = AppState {
        db_conn: conn,
        redis_conn: redis,
    };
    let app_status = Arc::new(AsyncRwLock::new(app_state));
    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .service(Fs::new("/static", concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
            .app_data(web::Data::new(Arc::clone(&app_status)))
            .wrap(middleware::Logger::default()) // enable logger
            .default_service(web::route())
            .configure(init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(logic::fetch_token::fetch_token);
    cfg.service(logic::union_api::api);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}



