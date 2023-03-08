use crate::base::{
    column::{BaseColumn, ColumnDefault, ColumnType},
    helper::generate_ulid,
    query::{Condition, Operator, QueryBuilder, WhereJoinOperator},
    query_values::Value,
    schema::SchemaManagerTrait,
    table::BaseTable,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use sqlx::{any::AnyKind, mysql::MySqlRow, types::chrono, Column, MySql, Pool, Row};
use std::sync::Arc;

struct ActiveQuery {
    statement: String,
    params: Vec<String>,
}
pub struct MySqlSchemaManager {
    db_pool: Arc<Pool<MySql>>,
    active_query: Option<ActiveQuery>,
}

impl MySqlSchemaManager {
    pub fn new(db_pool: Arc<Pool<MySql>>) -> Self {
        Self {
            db_pool,
            active_query: None,
        }
    }
}

#[async_trait]
impl SchemaManagerTrait for MySqlSchemaManager {
    fn instance(db_pool: Arc<Pool<MySql>>) -> Self
    where
        Self: Sized,
    {
        Self::new(db_pool)
    }

    fn kind(&self) -> AnyKind {
        AnyKind::MySql
        // self.db_pool.any_kind()
    }

    fn fetch_table_for_update(&self, name: &str) -> BaseTable {
        BaseTable::new(name)
    }
    async fn has_table(&self, name: &str) -> bool {
        let query = "SELECT table_name FROM INFORMATION_SCHEMA.TABLES WHERE table_name = ?";

        let result = sqlx::query(query)
            .bind(name)
            .map(|_row| true)
            .fetch_one(self.db_pool.as_ref())
            .await;

        result.unwrap_or(false)
    }

    async fn commit(&self, table: BaseTable) {
        self.do_commit(table).await
    }

    fn query(&mut self, query: QueryBuilder) -> &dyn SchemaManagerTrait
    where
        Self: Sized,
    {
        let mut params = Vec::new();
        let statement = self.build_query(&query, &mut params);

        self.active_query = Some(ActiveQuery { statement, params });

        self
    }

    async fn fetch_all_as_json(&self) -> Vec<serde_json::Value> {
        let mut results = Vec::new();
        match &self.active_query {
            Some(active_query) => {
                let mut query = sqlx::query(&active_query.statement);
                for p in &active_query.params {
                    query = query.bind::<&str>(p);
                }

                let mut rows = query.fetch(self.db_pool.as_ref());
                while let Some(row) = rows.try_next().await.ok().unwrap_or_default() {
                    results.push(self.row_to_json(&row));
                }
            }
            None => (),
        }

        results
    }
}

impl MySqlSchemaManager {
    async fn do_commit(&self, table: BaseTable) {
        if table.is_new() {
            println!("create new table");
            self.create_table(table).await
        } else {
            self.create_table(table).await
        }
    }

    async fn create_table(&self, table: BaseTable) {
        let columns: Vec<String> = table
            .columns()
            .iter()
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

        if self.has_table("_core_users").await {
            let new_user_query = "INSERT INTO `_core_users` (`id`, `username`, `email`)
VALUES (?, ?, ?);";

            let id = generate_ulid();

            let result = sqlx::query(new_user_query)
                .bind(&id)
                .bind("first_user")
                .bind(34)
                .execute(self.db_pool.as_ref())
                .await;
            match result {
                Ok(_) => {
                    println!("user was created")
                }
                Err(e) => {
                    dbg!(e.to_string());
                }
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
                ColumnDefault::Zero => the_type.push('0'),
            };
        }

        // column relationship
        if let Some(relationship) = &column.relationship {
            the_type.push_str(&format!(
                ", FOREIGN KEY (`{}`) REFERENCES `{}` (`{}`)",
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

    fn build_query(&self, query: &QueryBuilder, params: &mut Vec<String>) -> String {
        let mut sql = "SELECT".to_owned();

        // fields
        match query.select_columns() {
            Some(fields) => sql = format!("{} {}", sql, fields.join(",")),
            None => sql = format!("{} *", sql),
        };

        // from
        sql = format!("{} FROM {}", sql, query.tables().join(","));

        // joins

        // wheres
        sql = format!("{} {}", sql, self.build_where_clauses(query, params));

        sql
    }

    fn build_where_clauses(&self, query: &QueryBuilder, params: &mut Vec<String>) -> String {
        let mut wheres = "".to_owned();
        for where_join in query.where_clauses() {
            match where_join {
                WhereJoinOperator::And(condition) if !wheres.is_empty() => {
                    wheres = format!(
                        "{} AND {} ",
                        wheres,
                        self.transform_condition(condition, params)
                    );
                }
                WhereJoinOperator::Or(condition) if !wheres.is_empty() => {
                    wheres = format!(
                        "{} OR {} ",
                        wheres,
                        self.transform_condition(condition, params)
                    );
                }
                WhereJoinOperator::None(condition) => {
                    wheres = format!(
                        "{} {} ",
                        wheres,
                        self.transform_condition(condition, params)
                    );
                }
                WhereJoinOperator::And(condition) | WhereJoinOperator::Or(condition) => {
                    wheres = format!(
                        "{} {} ",
                        wheres,
                        self.transform_condition(condition, params)
                    );
                }
            }
        }

        if !wheres.is_empty() {
            wheres = format!("WHERE {}", wheres);
        }

        wheres
    }

    fn transform_condition(&self, condition: &Condition, params: &mut Vec<String>) -> String {
        self.transform_value(condition.value(), params);
        match condition.operator() {
            Operator::Equal => format!("{} = ?", condition.column()),
            Operator::NotEqual => format!("{} <> ?", condition.column()),
            Operator::Greater => format!("{} > ?", condition.column()),
            Operator::NotGreater => format!("NOT {} > ?", condition.column()),
            Operator::GreaterOrEqual => format!("{} >= ?", condition.column()),
            Operator::NotGreaterOrEqual => format!("NOT {} >= ?", condition.column()),
            Operator::Less => format!("{} < ?", condition.column()),
            Operator::NotLess => format!("NOT {} < ?", condition.column()),
            Operator::LessOrEqual => format!("{} <= ?", condition.column()),
            Operator::NotLessOrEqual => format!("NOT {} <= ?", condition.column()),
            Operator::Like => format!("{} like ?", condition.column()),
            Operator::NotLike => format!("NOT {} like ?", condition.column()),
            Operator::Null => format!("{} IS NULL", condition.column()),
            Operator::NotNull => format!("{} IS NOT NULL", condition.column()),
            Operator::In | Operator::NotIn => {
                let length = match &condition.value() {
                    Value::I64s(v) => v.len(),
                    Value::U64s(v) => v.len(),
                    Value::Strings(v) => v.len(),
                    _ => 1,
                };

                let mut placeholder = Vec::new();
                placeholder.resize(length, "?");

                if Operator::In == *condition.operator() {
                    format!("{} IN ({})", condition.column(), placeholder.join(","))
                } else {
                    format!("{} NOT IN ({})", condition.column(), placeholder.join(","))
                }
            }
        }
    }

    fn transform_value(&self, value: &Value, params: &mut Vec<String>) {
        match value {
            Value::Null => (),
            Value::U64(v) => params.push(v.to_string()),
            Value::I64(v) => params.push(v.to_string()),
            Value::F64(v) => params.push(v.to_string()),
            Value::String(v) => params.push(format!("'{}'", v)),
            Value::Boolean(v) => {
                params.push(if *v { 1.to_string() } else { 0.to_string() });
            }
            Value::U64s(v) => params.push(format!(
                "({})",
                v.as_slice()
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )),
            Value::I64s(v) => params.extend(
                v.as_slice()
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            ),
            Value::F64s(v) => params.extend(
                v.as_slice()
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            ),
            Value::Strings(v) => {
                let s = v
                    .iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<String>>();
                params.extend(s);
            }
            Value::SubQuery(q) => {
                self.build_query(q, params);
            }
        }
    }

    fn row_to_json(&self, row: &MySqlRow) -> serde_json::Value {
        let mut this_row = serde_json::Map::new();

        for col in row.columns() {
            match col.type_info().to_string().as_str() {
                "CHAR" | "VARCHAR" | "TEXT" => {
                    if let Ok(v) = row.try_get::<String, &str>(col.name()) {
                        this_row.insert(col.name().to_owned(), serde_json::Value::String(v));
                    } else {
                        this_row.insert(col.name().to_owned(), serde_json::Value::Null);
                    }
                }
                "BOOLEAN" => {
                    let v: bool = row.get(col.name());
                    this_row.insert(col.name().to_owned(), serde_json::Value::Bool(v));
                }
                "BIGINT UNSIGNED" => {
                    let x: u64 = row.get(col.name());
                    this_row.insert(
                        col.name().to_owned(),
                        serde_json::from_str(x.to_string().as_str()).unwrap(),
                    );
                }
                "BIGINT" => {
                    let v: i64 = row.get(col.name());

                    this_row.insert(
                        col.name().to_owned(),
                        serde_json::from_str(v.to_string().as_str()).unwrap(),
                    );
                }
                "DATETIME" => {
                    let v = row.try_get::<chrono::NaiveDateTime, &str>(col.name());

                    if let Ok(v) = v {
                        this_row.insert(
                            col.name().to_owned(),
                            serde_json::Value::String(v.to_string()),
                        );
                    } else {
                        this_row.insert(col.name().to_owned(), serde_json::Value::Null);
                    }
                }
                _ => {
                    dbg!("not mapped {:#?}", col.type_info());
                }
            }
        }

        serde_json::Value::Object(this_row)
    }
}
