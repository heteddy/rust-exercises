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
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Map, Number, Value};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMap2(serde_json::Map<String, serde_json::Value>);

impl BodyMap2 {
    pub fn new() -> Self {
        BodyMap2(serde_json::Map::new())
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_handlebars() -> Result<(), Box<dyn Error>> {
        let mut reg = Handlebars::new();
        // render without register
        println!(
            "{}",
            reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
        );
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
        bm2.0
            .insert("online".to_string(), serde_json::Value::Bool(true));
        bm2.0.insert(
            "title".to_string(),
            Value::Array(vec![
                Value::String("高级经理".to_string()),
                Value::String("技术专家".to_string()),
            ]),
        );
        /*
        Hello {{name}}  age={{ age }} salary={{ salary }}
        {{#if online }}
        online is true
        {{/if}}
        {{#each title}}
        {{this}}
        {{/each}}
        */
        let tpl_str = r#"  Hello {{name}}  age={{ age }} salary={{ salary }}
        {{#if online }}
        online is true
        {{/if}}  {{#each title}} {{this}};\n {{/each}}"#;
        // register template using given name
        reg.register_template_string("tpl_1", tpl_str)?;
        
        let render_str = reg.render("tpl_1", &bm2)?;
        println!("tpl_1={},", &render_str);
        print_type_of(&render_str);

        Ok(())
    }
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
