#[derive(Debug)]
enum Operator {
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Like,
    IsNull,
    In,
    Between,
}

#[derive(Debug)]
enum Action {
    Insert,
    Update,
    Delete,
    SelectAll,
    SelectOne,
}

#[derive(Debug)]
pub enum WhereJoinOperator {
    None(Condition),
    And(Condition),
    Or(Condition),
    Not(Condition),
    AndNot(Condition),
    OrNot(Condition),
}

#[derive(Debug)]
pub enum Value {
    Null,
    Number(f64),
    Numbers(Vec<f64>),
    Text(String),
    Texts(Vec<String>),
}

#[derive(Debug)]
pub struct Condition {
    column: String,
    operator: Operator,
    value: Value,
}

#[derive(Debug)]
pub struct QueryBuilder {
    where_clauses: Vec<WhereJoinOperator>,
    tables: Vec<String>,
    select_columns: Option<Vec<String>>,
    set_columns: Option<Vec<String>>,
}

impl QueryBuilder {
    pub fn new(tables: Vec<String>) -> Self {
        Self {
            where_clauses: Vec::new(),
            tables,
            select_columns: None,
            set_columns: None,
        }
    }

    pub fn where_(&mut self, where_clause: WhereJoinOperator) -> &mut Self {
        self.where_clauses.push(where_clause);
        self
    }
}
