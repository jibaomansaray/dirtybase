use async_trait::async_trait;
use sqlx::{any::AnyKind, Any, Pool};
use std::sync::Arc;

use super::{query::QueryBuilder, table::BaseTable};

#[async_trait]
pub trait SchemaManagerTrait {
    fn instance(db_pool: Arc<Pool<Any>>) -> Self
    where
        Self: Sized;

    fn kind(&self) -> AnyKind;

    // update an existing table
    fn fetch_table_for_update(&self, name: &str) -> BaseTable;

    // commit schema changes
    async fn commit(&self, table: BaseTable);

    async fn query(&self, query_builder: QueryBuilder);

    // checks if a table exist in the database
    async fn has_table(&self, name: &str) -> bool;
}
