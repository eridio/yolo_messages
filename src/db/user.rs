use crate::{config::crypto::CryptoService,
    models::user::{User,MessageBundle,MessageBundleStringify,MessageBundleStringify2},
    errors::AppError,
    errors::AppErrorCode,
    };


use actix_web::{web::Data, FromRequest ,HttpResponse, web::Json};
use sqlx::{PgPool, postgres::PgQueryAs,query_as};
use std::sync::Arc;
use std::ops::Deref;
use color_eyre::Result;
use uuid::Uuid;
use futures::future::{ready,Ready};
use tracing::instrument;
use serde_json::{Value,json};


pub struct UserRepository {
    pool: Arc<PgPool>
}

pub fn parse_bundle_arguments(s:String)-> Vec<u8> {          
         let mut s = s.replace("[", "");
         let mut s = s.replace("]", "");
         let t = s.replace(" ",""); //enlever les spaces
         let a : Vec<String> = t.split(",").map(str::to_string).collect();
         let mut vec : Vec<u8> = Vec::new();
         for i in 0..a.len() {
             vec.push(a[i].parse::<u8>().unwrap())
         }
         vec
}


impl UserRepository {
    pub fn new(pool:Arc<PgPool>) -> Self {
        Self {pool}
    }

    #[instrument(skip(self))]
    pub async fn find_by_username(&self, username: &str) -> Result<User> {
        let maybe_user = sqlx::query_as::<_, User>("select * from users_info where username = $1")
            .bind(username)
            .fetch_one(&*self.pool)
            .await?;

        Ok(maybe_user)
    }

    pub async fn store_bundle_db(&self, uuid : Uuid,message_to_store : MessageBundle )-> Result<MessageBundleStringify>{

        println!("{:?}", message_to_store);
        let bundle1 = sqlx::query_as::<_, MessageBundleStringify>("insert into yolo_message (uuid1,username,cipherText,key,nonce,conversationName,date) values ($1,$2,$3,$4,$5,$6,$7) returning *")
        .bind(uuid)
        .bind(format!("{:?}",message_to_store.username))
        .bind(format!("{:?}",message_to_store.cipherText))
        .bind(format!("{:?}",message_to_store.key))
        .bind(format!("{:?}",message_to_store.nonce))
        .bind(format!("{:?}",message_to_store.conversationName))
        .bind(format!("{:?}",message_to_store.date))
        .fetch_one(&*self.pool)
        .await?;
        println!("{:?}" , bundle1);
        let bundle_to_return_to_user = MessageBundleStringify {
            uuid1: bundle1.uuid1,
            username : bundle1.username,
            cipherText : (bundle1.cipherText),
            key: (bundle1.key),
            nonce : bundle1.nonce,
            conversationName : bundle1.conversationName,
            date : bundle1.date
        };
        Ok(bundle_to_return_to_user)
    }


    pub async fn get_messages_db(&self , id : Uuid) -> Result<Vec<MessageBundleStringify2>> {
        println!("dans get messages");

        let bundle_to_return = sqlx::query_as::<_, MessageBundleStringify2>("select * from yolo_message where uuid1 = $1 ;")//"select * from yolo_bundle where name_ = '$1';")
        .bind(id)
        .fetch_all(&*self.pool)
        .await;
        println!("{:?}", bundle_to_return);
    
    let bundle_to_return_stringify = bundle_to_return.unwrap();

    println!("{:?}", bundle_to_return_stringify);

        // let bundle_to_return_to_user = MessageBundle {
        //     uuid1: id,
        //     username : bundle_to_return_stringify.username,
        //     cipherText : parse_bundle_arguments(bundle_to_return_stringify.ciphertext),
        //     key: parse_bundle_arguments(bundle_to_return_stringify.key),
        //     nonce : bundle_to_return_stringify.nonce,
        //     conversationName : bundle_to_return_stringify.conversationname,
        //     date : bundle_to_return_stringify.date
        // };

        // println!("{:?}", bundle_to_return_to_user);

    Ok(bundle_to_return_stringify)
       
    }
}

impl FromRequest for UserRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req, payload))]
    fn from_request(    
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PgPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(UserRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}