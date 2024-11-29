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

use handlebars::{self, Handlebars, JsonRender, JsonValue};
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

        reg.register_helper(
            "stringify",
            Box::new(
                |h: &handlebars::Helper,
                 r: &Handlebars,
                 _: &handlebars::Context,
                 rc: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output|
                 -> handlebars::HelperResult {
                    let param =
                        h.param(0)
                            .ok_or(handlebars::RenderErrorReason::ParamNotFoundForIndex(
                                "closure-helper",
                                0,
                            ))?;
                    println!("{:?}", &param);
                    // out.write("3rd helper: ")?;
                    // let v = param.value();
                    let v_result = serde_json::to_string_pretty(param.value());
                    let v = v_result.unwrap();

                    out.write(&v)?;
                    Ok(())
                },
            ),
        );

        // render without register
        println!(
            "{}",
            reg.render_template(
                "Hello {{stringify title}}, i am {{name}}-{{stringify name}}, struct= {{stringify s}}",
                &json!({"title": ["a","b"],"name":"teddy", "s":{"s1":1,"s2":2}})
            )?
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
