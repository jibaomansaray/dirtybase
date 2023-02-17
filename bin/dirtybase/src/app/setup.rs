use dirtybase_orm::base;
use dotenv::dotenv;
use sqlx::{any::AnyPoolOptions, Any, Pool};
use std::{env, sync::Arc};

use super::db_setup::{self, create_data_tables};

pub struct Dirtybase {
    db_pool: Arc<Pool<Any>>,
}

impl Dirtybase {
    pub async fn new() -> anyhow::Result<Self> {
        let mut database_connection = "".to_owned();

        match dotenv() {
            Err(e) => {
                panic!("could not load .env file: {:#}", e);
            }
            _ => {
                // database connection string
                if let Ok(db_connection) = env::var("DATABASE") {
                    database_connection = db_connection;
                }
            }
        }

        Ok(Self {
            db_pool: Arc::new(db_connect(&database_connection).await),
        })
    }

    pub fn schema_manger(&self) -> base::manager::Manager {
        base::manager::Manager::new(self.db_pool.clone())
    }

    pub async fn db_setup(&self) {
        create_data_tables(&self.schema_manger()).await;
    }
}

pub async fn db_connect(conn: &str) -> Pool<Any> {
    match AnyPoolOptions::new().max_connections(5).connect(conn).await {
        Ok(conn) => conn,
        Err(e) => {
            panic!("could not connect to the database: {:#?}", e);
        }
    }
}
