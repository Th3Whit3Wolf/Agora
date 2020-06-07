extern crate agora_server;
#[macro_use] extern crate lazy_static;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use actix::prelude::*;
use actix_web::body::Body;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::CONTENT_TYPE;
use actix_web::http::{header::CACHE_CONTROL, HeaderValue};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use agora_server::settings::Settings;
//use regex::Regex;
use std::{io, sync::Arc};
//use tokio::sync::Mutex;
use color_eyre::{Help, Report};
use eyre::WrapErr;
use tracing::{instrument};
use std::env;

#[actix_rt::main]
#[instrument]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    pretty_env_logger::init();
    let settings = Settings::get();

    info!(
        "Starting http server at {}:{}",
        settings.bind, settings.port
    );

    HttpServer::new(move || {
        let settings = Settings::get();
        App::new()
        // enable logger - always register actix-web Logger middleware last
        .wrap(middleware::Logger::default())
    })
        .bind((settings.bind, settings.port))
        .wrap_err(format!("Unable to bind to {}:{}", settings.bind, settings.port))
        .suggestion("try using a different port")
        .unwrap()
        .run()
        .await
    
}

