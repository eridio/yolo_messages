use super::{authentication::AuthenticatedUser, AppResponse};
use crate::{db,
            config::crypto::CryptoService,
            db::user::UserRepository,
            errors::AppError,
            models::user::{MessageBundle,MessageBundleFromWeb}};

use actix_web::{web::Data,web::Json,web::Form, HttpResponse,HttpRequest,Responder,HttpMessage,web::Query};
use color_eyre::Result;
use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument};
use validator::Validate;
use serde_json::Value;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use uuid::Uuid;
use serde::{Deserialize,Serialize};

pub async fn test (req : HttpRequest) -> impl Responder{
    let request = req;
    HttpResponse::Ok().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","*").body(format!("{:?}",request))
}


//user : AuthenticatedUser,repository: UserRepository, 
pub async fn store_messages (user : AuthenticatedUser,repository: UserRepository,info : Json<MessageBundleFromWeb>) -> impl Responder{
   println!("{:?}", info);
    let message_bundle = MessageBundle {
        uuid1 : user.0,
        username : info.username.clone(),
        cipherText : info.cipherText.clone(),
        key : info.key.clone(),
        conversationName : info.conversationName.clone(),
        nonce: info.nonce.clone(),
        date : info.date.clone()
    };
    println!("{:?}" , message_bundle);

    let response_from_db = repository.store_bundle_db(user.0, message_bundle).await;

    HttpResponse::Ok().body("")
}

pub async fn get_messages(user:AuthenticatedUser, repository : UserRepository)  -> impl Responder{

    let json_to_return = repository.get_messages_db(user.0).await.unwrap();

    HttpResponse::Ok().json(json_to_return)
}