use std::option::Option;
use listenfd::ListenFd;
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::env;


use actix_files::Files as Fs;
use actix_web::{
    App, error, Error, get, HttpRequest, HttpResponse, HttpServer, middleware, post, Result, web,
};

use morioka_service::{
    sea_orm::{Database, DatabaseConnection},
    Query,
};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MoriokaParams {
    token: Option<String>,
    udid: Option<String>,
    data:Option<String>,
}


#[actix_web::main]
async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState {  conn };
    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .service(Fs::new("/static", concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
            .app_data(web::Data::new(state.clone()))
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
    cfg.service(fetch_token);
    cfg.service(api);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

#[get("/fetch_token/{mix_id}")]
async fn fetch_token(
    data: web::Data<AppState>,
    mix_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = &mix_id;
    let token = Query::fetch_token_by_mix(conn,id.to_string()).await.unwrap_or(Option::Some("".to_string()));
    if let Some(a) = token {
        Ok(HttpResponse::Ok().json(json!({"token": a}).clone()))
    } else {
        Ok(HttpResponse::Ok().json("{}"))
    }
}

#[post("/api")]
async fn api(
    body: web::Bytes,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    println!("enter to api");
    let item = serde_json::from_slice::<MoriokaParams>(&body)?;
    println!("model: {item:?}");

    Ok(HttpResponse::Ok().json("{}"))
}