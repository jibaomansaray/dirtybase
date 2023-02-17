use async_trait::async_trait;
use sqlx::{any::AnyKind, Any, Pool};
use std::sync::Arc;

use super::table::BaseTable;

pub struct QueryBuilder {
    pub tables: Vec<String>,
}

impl QueryBuilder {
    pub fn where_eq(&mut self) -> &mut Self {
        self
    }
    pub fn get(&mut self) -> bool {
        true
    }
}

#[async_trait]
pub trait SchemaManagerTrait {
    fn instance(db_pool: Arc<Pool<Any>>) -> Self
    where
        Self: Sized;

    fn kind(&self) -> AnyKind;

    // update an existing table
    fn fetch_table_for_update(&self, name: &str) -> BaseTable;

    fn query(&self, name: &str) -> QueryBuilder;

    // commit schema changes
    async fn commit(&self, table: BaseTable);

    // checks if a table exist in the database
    async fn has_table(&self, name: &str) -> bool;
}
