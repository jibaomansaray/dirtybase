pub(crate) struct MysqlTableManager<'a> {
    manager: &'a MySqlSchemaManager,
    table: BaseTable,
}

impl MysqlTableManager<'a> {
    pub fn new(table: BaseTable, manager: &'a MySqlSchemaManager) -> Self {
        Self { manager, table }
    }

    pub fn commit(&self) {
        let query = "SELECT *
FROM INFORMATION_SCHEMA.COLUMNS
WHERE table_name = ?";
        let mut rows = sqlx::query(query)
            .bind(&table.name)
            .fetch(self.manager.db_pool.as_ref());

        while let Some(row) = rows.try_next().await.expect("could not get all countries") {
            let name: String = row.try_get("COLUMN_NAME").unwrap();
            dbg!(name);
        }
    }
}
