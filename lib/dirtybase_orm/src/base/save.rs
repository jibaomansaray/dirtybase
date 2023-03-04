use std::collections::HashMap;

pub struct SaveRecord {
    table: String,
    columns: HashMap<String, String>,
}

impl SaveRecord {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_owned(),
            columns: HashMap::new(),
        }
    }

    pub fn set(&mut self, column: &str, value: String) -> &mut Self {
        self.columns.insert(column.to_owned(), value);
        self
    }

    pub fn set_many(&mut self, key_values: HashMap<String, String>) -> &mut Self {
        self.columns.extend(key_values);
        self
    }

    pub async fn save(&self) {
        match self.columns.get("internal_id") {
            Some(id) => println!("updating record with ID: {} in {}", id, &self.table),
            None => println!("Inserting new record into {}", &self.table),
        }
    }
}
