use actix_web::{web, web::ServiceConfig, HttpResponse};
mod authentication;
mod user;
use crate::errors::AppError;


use user::{test,get_messages,store_messages};



type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;


pub fn app_config(config : &mut ServiceConfig) {



    let test = web::resource("/test").route(web::get().to(test));


    let store_messages = web::resource("/store_messages").route(web::post().to(store_messages));

    let get_messages = web::resource("/get_messages").route(web::get().to(get_messages));

    config.service(test).service(get_messages).service(store_messages);
}