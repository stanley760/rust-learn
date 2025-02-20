use std::fmt::{Display, Formatter};
use abi::{command_request::RequestData, *};
use http::StatusCode;
use crate::error::kv::KvError;

pub mod abi;

impl CommandRequest {
    // COMMAND HGET
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            }))
        }
    }

    // COMMAND HSET
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    // COMMAND HGETALL
    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            }))
        }
    }


}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self { key: key.into(), value: Some(value) }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            value: Some(value::Value::String(value)),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(value)),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            value: Some(value::Value::String(value.into())),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value.as_ref() {
            Some(value::Value::String(s)) => write!(f, "{}", s),
            Some(value::Value::Integer(i)) => write!(f, "{}", i),
            Some(value::Value::Bool(b)) => write!(f, "{}", b),
            Some(value::Value::Binary(b)) => write!(f, "{:?}", b), // 使用 Debug 格式化
            Some(value::Value::Float(fl)) => write!(f, "{}", fl),
            None => write!(f, "None"),
        }
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(data: (String, Value)) -> Self {
        Kvpair::new(data.0, data.1)
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![value],
            ..Default::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse  {
    fn from(value: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: value,
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut res = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _, 
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };
        
        match e {
            KvError::NotFound(_, _) => res.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => res.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }
        res
    }
}