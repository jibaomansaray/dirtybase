use crate::base::{
    column::{BaseColumn, ColumnDefault, ColumnType},
    schema::{QueryBuilder, SchemaManagerTrait},
    table::BaseTable,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use sqlx::{any::AnyKind, Any, Pool, Row};
use std::{fmt::format, sync::Arc};

pub struct MySqlSchemaManager {
    db_pool: Arc<Pool<Any>>,
}

impl MySqlSchemaManager {
    pub fn new(db_pool: Arc<Pool<Any>>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl SchemaManagerTrait for MySqlSchemaManager {
    fn instance(db_pool: Arc<Pool<Any>>) -> Self
    where
        Self: Sized,
    {
        Self::new(db_pool)
    }

    fn kind(&self) -> AnyKind {
        self.db_pool.any_kind()
    }

    fn fetch_table_for_update(&self, name: &str) -> BaseTable {
        BaseTable::new(name)
    }
    async fn has_table(&self, name: &str) -> bool {
        let query = "SELECT table_name FROM INFORMATION_SCHEMA.TABLES WHERE table_name = ?";

        let result = sqlx::query(&query)
            .bind(name)
            .map(|_row| true)
            .fetch_one(self.db_pool.as_ref())
            .await;

        match result {
            Ok(exist) => exist,
            Err(_) => false,
        }
    }

    async fn commit(&self, table: BaseTable) {
        //         let query = "SELECT *
        // FROM INFORMATION_SCHEMA.COLUMNS
        // WHERE table_name = ?";
        //         let mut rows = sqlx::query(query)
        //             .bind(&table.name)
        //             .fetch(self.db_pool.as_ref());

        //         while let Some(row) = rows.try_next().await.expect("could not get all countries") {
        //             let name: String = row.try_get("COLUMN_NAME").unwrap();
        //             dbg!(name);
        //         }
        self.do_commnit(table).await
    }

    fn query(&self, name: &str) -> QueryBuilder {
        QueryBuilder {
            tables: vec![name.to_owned()],
        }
    }
}

impl MySqlSchemaManager {
    async fn do_commnit(&self, table: BaseTable) {
        if table.is_new() {
            println!("create new table");
        } else {
            self.create_table(table).await
        }
    }

    async fn create_table(&self, table: BaseTable) {
        let columns: Vec<String> = table
            .columns()
            .into_iter()
            .map(|column| self.create_column(column))
            .collect();

        let query = format!(
            "CREATE TABLE `{}` (\n{}\n) ENGINE='InnoDB';",
            &table.name,
            columns.join(",\n")
        );

        println!("{}", &query);

        let result = sqlx::query(&query).execute(self.db_pool.as_ref()).await;

        match result {
            Ok(x) => {
                println!("----------------------- ok result -------------");
                dbg!(x);
                println!("----------------------- ok result -------------");
            }
            Err(e) => {
                println!("----------------------- error result -------------");
                dbg!(e.to_string());
                println!("----------------------- error result -------------");
            }
        }

        println!("update existing table");
        dbg!(query);
    }

    fn create_column(&self, column: &BaseColumn) -> String {
        let mut entry = format!("`{}`", &column.name);
        let mut the_type = " ".to_owned();

        // column type
        match column.column_type {
            ColumnType::AutoIncrementId => {
                the_type.push_str("bigint(20) unsigned AUTO_INCREMENT PRIMARY KEY")
            }
            ColumnType::Boolean => the_type.push_str("tinyint(1)"),
            ColumnType::Char(length) => {
                the_type.push_str(&format!("char({}) COLLATE 'utf8mb4_unicode_ci'", length))
            }
            ColumnType::Date => the_type.push_str("datetime"),
            // ColumnType::File() shouldn't be here
            // ColumnType::Float not sure
            ColumnType::Integer => the_type.push_str("bigint(20)"),
            ColumnType::Json => the_type.push_str("json"),
            ColumnType::Number => the_type.push_str("double"),
            // ColumnType::Relation { relation_type, table_name }
            // ColumnType::Select()
            ColumnType::String(length) => {
                let q = format!("varchar({}) COLLATE 'utf8mb4_unicode_ci'", length);
                the_type.push_str(q.as_str());
            }
            ColumnType::Text => the_type.push_str("longtext"),
            ColumnType::Uuid => the_type.push_str("uuid"),
            _ => the_type.push_str("varchar(255)"),
        };

        // column is nullable
        if let Some(nullable) = column.is_nullable {
            if nullable {
                the_type.push_str(" NULL");
            } else {
                the_type.push_str(" NOT NULL");
            }
        }

        // column is unique
        if column.is_unique {
            the_type.push_str(" UNIQUE");
        }

        // column default
        if let Some(default) = &column.default {
            the_type.push_str(" DEFAULT ");
            match default {
                ColumnDefault::CreatedAt => the_type.push_str("now()"),
                ColumnDefault::Custom(d) => the_type.push_str(&format!("'{}'", d)),
                ColumnDefault::EmptyArray => the_type.push_str("[]"),
                ColumnDefault::EmptyObject => the_type.push_str("{}"),
                ColumnDefault::EmptyString => the_type.push_str(""),
                ColumnDefault::Uuid => the_type.push_str("SYS_GUID()"),
                ColumnDefault::Ulid => (),
                ColumnDefault::UpdatedAt => {
                    the_type.push_str("current_timestamp() ON UPDATE CURRENT_TIMESTAMP")
                }
                ColumnDefault::Zero => the_type.push_str("0"),
            };
        }

        // column relationsip
        if let Some(relationship) = &column.relationship {
            the_type.push_str(&format!(
                ", FOREIGN KEY (`{}`) REFERENCES `{}` (`{}`) ON DELETE CASCADE",
                &column.name,
                &relationship.table(),
                &relationship.column()
            ));
            if relationship.cascade_delete() {
                the_type.push_str(" ON DELETE CASCADE");
            }
        }

        entry.push_str(&the_type);
        entry
    }
}
