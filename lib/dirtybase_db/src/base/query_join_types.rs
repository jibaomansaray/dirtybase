use std::fmt::Display;

#[derive(Debug)]
pub enum JoinType {
    Inner,
    Left,
    Right,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match &self {
            Self::Inner => "inner",
            Self::Left => "left",
            Self::Right => "right",
        };
        write!(f, "{}", name)
    }
}
