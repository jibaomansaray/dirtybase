use super::{query_operators::Operator, query_values::Value};

#[derive(Debug)]
pub struct Condition {
    pub column: String,
    pub operator: Operator,
    pub value: Value,
}

impl Condition {
    pub fn new<T: Into<Value>>(column: &str, operator: Operator, value: T) -> Self {
        Self {
            column: column.to_owned(),
            operator,
            value: value.into(),
        }
    }

    pub fn column(&self) -> &String {
        &self.column
    }

    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
