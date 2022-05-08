mod config;
mod db;
mod errors;
mod handlers;
mod models;
use handlers::app_config;


extern crate serde;
use crate::config::Config;
//use chrono::{DateTime, Utc};
use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};


 #[actix_web::main]
 async fn main() -> std::io::Result<()> {


    //config
        let config : Config= Config::from_env()
            .expect("error while server configuration");

    //pool (allow connection to be reuse for futures requests)

    let pool = config.db_pool().await.expect("pool error");

    //init the crypto service 

    let crypto_service = config.crypto_service();


     HttpServer::new( move || {
         App::new()
                .wrap(Logger::default())
                .data(pool.clone())
                .data(crypto_service.clone())
                .configure(app_config)
     })         

     .bind(format!("{}:{}",config.host,config.port))?
     .run()
     .await
 }
