pub mod crypto;
use crypto::CryptoService;

use color_eyre::Result;
use serde::Deserialize;
use dotenv::dotenv;
use eyre::WrapErr;
use tracing::{info, instrument};
use tracing_subscriber::{EnvFilter};
use sqlx::postgres::PgPool;
use std::time::Duration;
use std::sync::Arc;



#[derive(Debug, Deserialize)]
pub struct Config {
    pub host : String,
    pub port : i32,
    pub database_url : String,
    pub secret_key : String,
    pub jwt_secret: String,
}

impl Config {


    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("loading config"); //create a tracing event

        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;

        c.try_into()
                .context("error while loading config from env") // convert it into an instance of our config struct
    } 

    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("creatign database pool");

        PgPool::builder()
            .connect_timeout(Duration::from_secs(30))
            .build(&self.database_url)
            .await
            .context("creating db connexion pool")
    }
//init the crypto service

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService {
            key : Arc::new(self.secret_key.clone()),
            jwt_secret: Arc::new(self.jwt_secret.clone()),
        }
    }
}