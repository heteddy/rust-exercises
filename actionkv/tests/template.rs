use serde::Serialize;

use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
    name: String,
}

static TEMPLATE: &'static str = "Hello {name}!";

pub mod tests {
    use super::*;
    #[test]
    fn test_template() -> Result<(), Box<dyn Error>> {
        let mut tt = TinyTemplate::new();
        tt.add_template("hello", TEMPLATE)?;

        let context = Context {
            name: "World".to_string(),
        };

        let rendered = tt.render("hello", &context)?;
        println!("{}", rendered);

        Ok(())
    }
}
