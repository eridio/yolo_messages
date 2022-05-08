use color_eyre::Result;
use std::sync::Arc;
use argonautica::{Hasher,Verifier};
use futures::compat::Future01CompatExt;
use eyre::eyre;
use tracing::instrument;
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use actix_web::web::block;
use chrono::{Duration, Utc};

#[derive(Debug,Clone)]
pub struct CryptoService{
    pub key : Arc<String>,
    pub jwt_secret : Arc<String>
}

#[derive(Serialize,Deserialize)]
pub struct Permissions{
    pub sub: Uuid,
    pub exp: i64
}
#[derive(Serialize)]
pub struct Auth {
    pub token: String,
}

impl CryptoService {
    #[instrument(skip(self, token))]
    pub async fn verify_jwt(&self, token: String) -> Result<TokenData<Permissions>> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
            let validation = Validation::default();
            decode::<Permissions>(&token, &decoding_key, &validation)
        })
        .await
        .map_err(|err| eyre!("Verifying jwt token: {}", err))
    }

    

}