use dirtybase_orm::base::schema::SchemaManagerTrait;
use dirtybase_orm::{base, driver::mysql::mysql_schema_manager::MySqlSchemaManager};
use dotenv::dotenv;
use sqlx::{any::AnyKind, mysql::MySqlPoolOptions, MySql, Pool};
use std::{env, str::FromStr, sync::Arc};

use super::setup_database::create_data_tables;

pub struct Dirtybase {
    db_pool: Arc<Pool<MySql>>,
    kind: AnyKind,
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

        let kind = AnyKind::from_str(&database_connection).unwrap_or_else(|_| AnyKind::MySql);
        let mut instance = Self {
            kind,
            db_pool: Arc::new(db_connect(&database_connection).await),
        };

        // match instance.kind {
        //     // @todo implement the other supported databases' driver
        //     _ => instance.mysql_pool = Some(Arc::new(db_connect(&instance.url).await)),
        // };

        Ok(instance)
    }

    pub fn schema_manger(&self) -> base::manager::Manager {
        match self.kind {
            _ => base::manager::Manager::new(Box::new(MySqlSchemaManager::instance(
                self.db_pool.clone(),
            ))),
        }
    }

    pub async fn db_setup(&self) {
        create_data_tables(&self.schema_manger()).await;
    }
}

pub async fn db_connect(conn: &str) -> Pool<MySql> {
    match MySqlPoolOptions::new()
        .max_connections(5)
        .connect(conn)
        .await
    {
        Ok(conn) => conn,
        Err(e) => {
            panic!("could not connect to the database: {:#?}", e);
        }
    }
}
