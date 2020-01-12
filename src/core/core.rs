use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DeprecatedValue {
    Int(i32),
    String(String),
    List(usize),
    Bool(bool),
    Number(i32, u32),
    // 9 decimal digits
    ListInt(Vec<i32>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
//#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Integer(i64),

    // can use simply Float(f64)
    // the trait `std::cmp::Eq` is not implemented for `f64`
    // integer/ decimals with 18 digits
    Float(i64, u64),
    // integer part, decimal part (9 digits) TODO Clarify your custom type
    String(String),
    List(Vec<Value>),
    Object(Vec<(String, Value)>),
    Nodeset(usize),
    Bytes(Vec<u8>),
    None,
}

impl Value {
    pub fn from_f64(value: f64) -> Value {
        let integer = value.round() as i64;
        let decimal = (value.abs().fract() * 1000000000000000000.0).round() as u64;
        return Value::Float(integer, decimal);
    }

    pub fn is_scalar(&self) -> bool {
        return match self {
            Value::Nodeset(_) | Value::List(_) => false,
            _ => true,
        };
    }
    pub fn to_string(&self) -> String {
        return match self {
            Value::Integer(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            Value::Float(int, dec) => format!("{}.{}", int, dec),
            Value::String(x) => x.to_string(),
            Value::List(values) => {
                let values : Vec<String> = values.iter().map(|e| e.to_string()).collect();
                format!("List({})", values.join(","))
            }
            Value::Object(_) => format!("Object()"),
            Value::Nodeset(x) => format!("Nodeset{:?}", x),
            Value::Bytes(x) => format!("Bytes({:?})", x),
            Value::None => format!("None"),
        };
    }
}

#[test]
fn test_from_f64() {
    assert_eq!(Value::from_f64(1.0), Value::Float(1, 0));
    assert_eq!(Value::from_f64(-1.0), Value::Float(-1, 0));
    assert_eq!(Value::from_f64(1.1), Value::Float(1, 100000000000000096)); //TBC!!
    assert_eq!(Value::from_f64(-1.1), Value::Float(-1, 100000000000000096));
}


#[test]
fn test_is_scalar() {
    assert_eq!(Value::Integer(1).is_scalar(), true);
    assert_eq!(Value::List(vec![]).is_scalar(), false);

}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceInfo {
    pub start: Pos,
    pub end: Pos,
}

impl SourceInfo {
    pub fn init(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_column: usize,
    ) -> SourceInfo {
        return SourceInfo {
            start: Pos {
                line: start_line,
                column: start_col,
            },
            end: Pos {
                line: end_line,
                column: end_column,
            },
        };
    }
}

pub trait FormatError {
    fn source_info(&self) -> SourceInfo;
    fn description(&self) -> String;
    fn fixme(&self) -> String;
}
