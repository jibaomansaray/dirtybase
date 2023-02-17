use super::{schema::SchemaManagerTrait, table::BaseTable};
use crate::driver::mysql::mysql_schema_manager::MySqlSchemaManager;
use sqlx::{any::AnyKind, Any, Pool};
use std::sync::Arc;

pub struct Manager {
    schema: Box<dyn SchemaManagerTrait>,
}

impl Manager {
    pub fn new(db_pool: Arc<Pool<Any>>) -> Self {
        let schema = match &db_pool.any_kind() {
            // @todo implement the other support databases' driver
            _ => Box::new(MySqlSchemaManager::instance(db_pool)),
        };

        Self { schema }
    }

    pub fn db_kind(&self) -> AnyKind {
        self.schema.kind()
    }

    pub fn is_mysql(&self) -> bool {
        self.db_kind() == AnyKind::MySql
    }

    pub fn inner(&mut self) -> &dyn SchemaManagerTrait {
        self.schema.as_mut()
    }

    // Get an existing table to updating
    pub async fn table(&self, name: &str, mut callback: impl FnMut(&mut BaseTable)) {
        let mut table = self.schema.fetch_table_for_update(name);
        table.set_is_new(false);

        callback(&mut table);
        self.schema.commit(table).await;
    }

    pub async fn has_table(&self, name: &str) -> bool {
        self.schema.has_table(name).await
    }
}
