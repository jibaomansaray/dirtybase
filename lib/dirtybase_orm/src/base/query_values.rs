#[derive(Debug)]
pub enum Value {
    Null,
    Usize(usize),
    Usizes(Vec<usize>),
    Isize(isize),
    Isizes(Vec<isize>),
    F64(f64),
    F64s(Vec<f64>),
    String(String),
    Strings(Vec<String>),
    Boolean(bool),
    Booleans(Vec<bool>),
}

// i32
impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Isize(value as isize)
    }
}

impl From<Vec<i32>> for Value {
    fn from(value: Vec<i32>) -> Self {
        Self::Isizes(value.into_iter().map(|x| x as isize).collect())
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Usize(value as usize)
    }
}

impl From<Vec<u32>> for Value {
    fn from(value: Vec<u32>) -> Self {
        Self::Usizes(value.into_iter().map(|x| x as usize).collect())
    }
}
