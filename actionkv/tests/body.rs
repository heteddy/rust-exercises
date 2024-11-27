#![cfg_attr(
    debug_assertions,
    allow(
        unused,
        dead_code,
        unused_imports,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]
use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Number, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyValue {
    Int64(i64),
    // #[serde(flatten)]
    Float64(f64),
    String(String),
    VecS(Vec<String>),
    VecI(Vec<i64>),
    VecF(Vec<f64>),
}

// pub trait QdrantValue

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMap(HashMap<String, BodyValue>);

impl BodyMap {
    pub fn new() -> Self {
        BodyMap(HashMap::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMap2(serde_json::Map<String, serde_json::Value>);

impl BodyMap2 {
    pub fn new() -> Self {
        BodyMap2(serde_json::Map::new())
    }
}

mod tests {
    use axum::body::Body;

    use super::*;

    #[test]
    fn test_enum_ser() {
        let mut bm = BodyMap::new();
        bm.0.insert("name".to_string(), BodyValue::String("hedetao".to_string()));
        bm.0.insert("age".to_string(), BodyValue::Int64(40));
        bm.0.insert("salary".to_string(), BodyValue::Float64(80.5));
        bm.0.insert(
            "title".to_string(),
            BodyValue::VecS(vec!["高级经理".to_string(), "技术转件".to_string()]),
        );

        let ret = serde_json::to_string_pretty(&bm).unwrap_or_default();
        println!("serde_result={}", ret);

        let mut bm2 = BodyMap2::new();
        bm2.0
            .insert("name".to_string(), Value::String("hedetao".to_string()));
        bm2.0.insert(
            "age".to_string(),
            serde_json::Value::Number(Number::from(40)),
        );
        bm2.0.insert(
            "salary".to_string(),
            serde_json::Value::Number(Number::from_f64(80.50_f64).unwrap()),
        );
        bm2.0.insert(
            "title".to_string(),
            Value::Array(vec![
                Value::String("高级经理".to_string()),
                Value::String("技术转件".to_string()),
            ]),
        );

        let ret = serde_json::to_string_pretty(&bm2).unwrap_or_default();
        println!("serde_result2={}", ret);

        let av = Value::Array(vec![
            Value::String("高级经理".to_string()),
            Value::String("技术转件".to_string()),
        ]);
        let av_ret = serde_json::to_string_pretty(&av).unwrap_or_default();

        println!("av_ret2={}", av_ret);
        print_type_of(&av_ret);
    }
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
