use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimpleValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<SimpleValue>),
    Object(HashMap<String, SimpleValue>),
}

impl SimpleValue {
    // 构造方法
    pub fn null() -> Self {
        SimpleValue::Null
    }

    pub fn bool(v: bool) -> Self {
        SimpleValue::Bool(v)
    }

    pub fn integer(v: i64) -> Self {
        SimpleValue::Integer(v)
    }

    pub fn float(v: f64) -> Self {
        SimpleValue::Float(v)
    }

    pub fn string<S: Into<String>>(v: S) -> Self {
        SimpleValue::String(v.into())
    }

    pub fn array(v: Vec<SimpleValue>) -> Self {
        SimpleValue::Array(v)
    }

    pub fn object(v: HashMap<String, SimpleValue>) -> Self {
        SimpleValue::Object(v)
    }

    // 类型检查方法
    pub fn is_null(&self) -> bool {
        matches!(self, SimpleValue::Null)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, SimpleValue::Bool(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, SimpleValue::Integer(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, SimpleValue::Float(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, SimpleValue::Integer(_) | SimpleValue::Float(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, SimpleValue::String(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, SimpleValue::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, SimpleValue::Object(_))
    }

    // 获取值方法
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SimpleValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            SimpleValue::Integer(v) => Some(*v),
            SimpleValue::Float(v) if v.fract() == 0.0 => Some(*v as i64),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            SimpleValue::Float(v) => Some(*v),
            SimpleValue::Integer(v) => Some(*v as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            SimpleValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<SimpleValue>> {
        match self {
            SimpleValue::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, SimpleValue>> {
        match self {
            SimpleValue::Object(v) => Some(v),
            _ => None,
        }
    }

    // 可变引用获取方法
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<SimpleValue>> {
        match self {
            SimpleValue::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, SimpleValue>> {
        match self {
            SimpleValue::Object(v) => Some(v),
            _ => None,
        }
    }

    // 转换方法
    pub fn from_json(value: &Value) -> Self {
        match value {
            Value::Null => SimpleValue::Null,
            Value::Bool(b) => SimpleValue::Bool(*b),
            Value::Number(n) => {
                if n.is_i64() {
                    SimpleValue::Integer(n.as_i64().unwrap())
                } else {
                    SimpleValue::Float(n.as_f64().unwrap_or(0.0))
                }
            },
            Value::String(s) => SimpleValue::String(s.clone()),
            Value::Array(arr) => SimpleValue::Array(
                arr.iter().map(SimpleValue::from_json).collect()
            ),
            Value::Object(obj) => SimpleValue::Object(
                obj.iter()
                    .map(|(k, v)| (k.clone(), SimpleValue::from_json(v)))
                    .collect()
            ),
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            SimpleValue::Null => Value::Null,
            SimpleValue::Bool(b) => Value::Bool(*b),
            SimpleValue::Integer(i) => Value::Number((*i).into()),
            SimpleValue::Float(f) => serde_json::Number::from_f64(*f)
                .map(Value::Number)
                .unwrap_or(Value::Null),
            SimpleValue::String(s) => Value::String(s.clone()),
            SimpleValue::Array(arr) => Value::Array(
                arr.iter().map(|v| v.to_json()).collect()
            ),
            SimpleValue::Object(obj) => Value::Object(
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect()
            ),
        }
    }

    // 对象操作方法
    pub fn get(&self, key: &str) -> Option<&SimpleValue> {
        match self {
            SimpleValue::Object(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut SimpleValue> {
        match self {
            SimpleValue::Object(map) => map.get_mut(key),
            _ => None,
        }
    }

    pub fn insert(&mut self, key: String, value: SimpleValue) -> Option<SimpleValue> {
        match self {
            SimpleValue::Object(map) => map.insert(key, value),
            _ => None,
        }
    }

    // 数组操作方法
    pub fn push(&mut self, value: SimpleValue) -> bool {
        match self {
            SimpleValue::Array(vec) => {
                vec.push(value);
                true
            }
            _ => false,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&SimpleValue> {
        match self {
            SimpleValue::Array(vec) => vec.get(index),
            _ => None,
        }
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut SimpleValue> {
        match self {
            SimpleValue::Array(vec) => vec.get_mut(index),
            _ => None,
        }
    }
}

// 实现 From trait 用于类型转换
impl From<bool> for SimpleValue {
    fn from(v: bool) -> Self {
        SimpleValue::Bool(v)
    }
}

impl From<i64> for SimpleValue {
    fn from(v: i64) -> Self {
        SimpleValue::Integer(v)
    }
}

impl From<f64> for SimpleValue {
    fn from(v: f64) -> Self {
        SimpleValue::Float(v)
    }
}

impl From<String> for SimpleValue {
    fn from(v: String) -> Self {
        SimpleValue::String(v)
    }
}

impl From<&str> for SimpleValue {
    fn from(v: &str) -> Self {
        SimpleValue::String(v.to_string())
    }
}

impl From<Vec<SimpleValue>> for SimpleValue {
    fn from(v: Vec<SimpleValue>) -> Self {
        SimpleValue::Array(v)
    }
}

impl From<HashMap<String, SimpleValue>> for SimpleValue {
    fn from(v: HashMap<String, SimpleValue>) -> Self {
        SimpleValue::Object(v)
    }
} 