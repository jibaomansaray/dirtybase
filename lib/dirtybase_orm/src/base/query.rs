use super::query_values::Value;

#[derive(Debug)]
pub enum Operator {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    NotLess,
    NotLessOrEqual,
    Like,
    NotLike,
    Null,
    NotNull,
    In,
    NotIn,
    Between,
    NotBetween,
}

#[derive(Debug)]
pub enum Action {
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
}

pub enum WhereJoin {
    And,
    Or,
}

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

    pub fn eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Equal, value, None)
    }

    pub fn and_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Equal, value, Some(WhereJoin::And))
    }

    pub fn or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Equal, value, Some(WhereJoin::Or))
    }

    pub fn not_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotEqual, value, None)
    }

    pub fn and_not_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotEqual, value, Some(WhereJoin::And))
    }

    pub fn or_not_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotEqual, value, Some(WhereJoin::Or))
    }

    pub fn gt<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Greater, value, None)
    }

    pub fn and_gt<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Greater, value, Some(WhereJoin::And))
    }

    pub fn or_gt<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Greater, value, Some(WhereJoin::Or))
    }

    pub fn ngt_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::GreaterOrEqual, value, None)
    }

    pub fn and_ngt_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(
            column,
            Operator::GreaterOrEqual,
            value,
            Some(WhereJoin::And),
        )
    }
    pub fn or_ngt_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::GreaterOrEqual, value, Some(WhereJoin::Or))
    }

    pub fn le<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Less, value, None)
    }

    pub fn and_le<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Less, value, Some(WhereJoin::And))
    }
    pub fn or_le<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Less, value, Some(WhereJoin::Or))
    }

    pub fn le_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::LessOrEqual, value, None)
    }

    pub fn and_le_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::LessOrEqual, value, Some(WhereJoin::And))
    }

    pub fn or_le_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::LessOrEqual, value, Some(WhereJoin::Or))
    }

    pub fn not_le<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotLess, value, None)
    }

    pub fn and_nle<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotLess, value, Some(WhereJoin::And))
    }

    pub fn or_nle<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotLess, value, Some(WhereJoin::Or))
    }

    pub fn nle_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotLessOrEqual, value, None)
    }

    pub fn and_nle_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(
            column,
            Operator::NotLessOrEqual,
            value,
            Some(WhereJoin::And),
        )
    }

    pub fn or_nle_or_eq<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotLessOrEqual, value, Some(WhereJoin::Or))
    }

    pub fn like<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Like, value, None)
    }

    pub fn and_like<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Like, value, Some(WhereJoin::And))
    }

    pub fn or_like<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::Like, value, Some(WhereJoin::Or))
    }

    pub fn nlike<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotLike, value, None)
    }

    pub fn and_nlike<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotLike, value, Some(WhereJoin::And))
    }

    pub fn or_nlike<T: Into<Value>>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::NotLike, value, Some(WhereJoin::Or))
    }

    pub fn is_null(&mut self, column: &str) -> &mut Self {
        self.where_operator(column, Operator::Null, Value::Null, None)
    }

    pub fn and_is_null(&mut self, column: &str) -> &mut Self {
        self.where_operator(column, Operator::Null, Value::Null, Some(WhereJoin::And))
    }

    pub fn or_is_null(&mut self, column: &str) -> &mut Self {
        self.where_operator(column, Operator::Null, Value::Null, Some(WhereJoin::Or))
    }

    pub fn is_not_null(&mut self, column: &str) -> &mut Self {
        self.where_operator(column, Operator::NotNull, Value::Null, None)
    }

    pub fn and_is_not_null(&mut self, column: &str) -> &mut Self {
        self.where_operator(column, Operator::NotNull, Value::Null, Some(WhereJoin::And))
    }

    pub fn or_is_not_null(&mut self, column: &str) -> &mut Self {
        self.where_operator(column, Operator::NotNull, Value::Null, Some(WhereJoin::Or))
    }

    pub fn is_in<T: Into<Value> + IntoIterator>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::In, value, None)
    }

    pub fn and_is_in<T: Into<Value> + IntoIterator>(
        &mut self,
        column: &str,
        value: T,
    ) -> &mut Self {
        self.where_operator(column, Operator::In, value, Some(WhereJoin::And))
    }

    pub fn or_is_in<T: Into<Value> + IntoIterator>(&mut self, column: &str, value: T) -> &mut Self {
        self.where_operator(column, Operator::In, value, Some(WhereJoin::Or))
    }

    pub fn is_not_in<T: Into<Value> + IntoIterator>(
        &mut self,
        column: &str,
        value: T,
    ) -> &mut Self {
        self.where_operator(column, Operator::NotIn, value, None)
    }

    pub fn and_is_not_in<T: Into<Value> + IntoIterator>(
        &mut self,
        column: &str,
        value: T,
    ) -> &mut Self {
        self.where_operator(column, Operator::NotIn, value, Some(WhereJoin::And))
    }

    pub fn or_is_not_in<T: Into<Value> + IntoIterator>(
        &mut self,
        column: &str,
        value: T,
    ) -> &mut Self {
        self.where_operator(column, Operator::NotIn, value, Some(WhereJoin::Or))
    }

    pub fn where_(&mut self, where_clause: WhereJoinOperator) -> &mut Self {
        self.where_clauses.push(where_clause);
        self
    }

    fn first_or_and(&mut self, condition: Condition) -> &mut Self {
        if self.where_clauses.is_empty() {
            self.where_(WhereJoinOperator::None(condition))
        } else {
            self.and_where(condition)
        }
    }

    pub fn where_operator<T: Into<Value>>(
        &mut self,
        column: &str,
        operator: Operator,
        value: T,
        and_or: Option<WhereJoin>,
    ) -> &mut Self {
        let condition = Condition::new(column, operator, value);
        match and_or {
            Some(j) => match j {
                WhereJoin::And => self.and_where(condition),
                WhereJoin::Or => self.or_where(condition),
            },
            _ => self.first_or_and(condition),
        }
    }

    fn or_where(&mut self, condition: Condition) -> &mut Self {
        self.where_(WhereJoinOperator::Or(condition))
    }

    fn and_where(&mut self, condition: Condition) -> &mut Self {
        self.where_(WhereJoinOperator::Or(condition))
    }
}
