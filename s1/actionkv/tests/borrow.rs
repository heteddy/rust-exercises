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
use std::hash::Hasher;

#[derive(Debug, Clone)]
struct App {
    pub app_id: String,
    pub app_secret: String,
    // 租户名称，
    pub tenant: String,
    // 联系人
    pub liaison: String,
    //子系统名称
    pub system: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            app_id: "".into(),
            app_secret: "".into(),
            tenant: "".into(),
            liaison: "".to_owned(),
            system: "".to_owned(),// 子系统编号
        }
    }
}


impl PartialEq<App> for App {
    fn eq(&self, other: &App) -> bool {
        self.app_id == other.app_id
    }
}

impl Eq for App {}

impl std::hash::Hash for App {
    fn hash<H: Hasher>(&self, state: &mut H)
        where H: Hasher
    {
        self.app_id.hash(state)
    }
}


/*
println!("{:?}",v.get("meeting_minutes").unwrap_or_default());
下面的报错
 --> tests/borrow.rs:97:37
    |
97  |         println!("{:?}",app_set.get("meeting_minutes3").unwrap_or(&App::default()));
    |                                 --- ^^^^^^^^^^^^^^^^^^ expected `&App`, found `&str`
    |                                 |
    |                                 arguments to this method are incorrect
    |
    = note: expected reference `&App`
               found reference `&'static str`


 */
impl std::borrow::Borrow<str> for App {
    fn borrow(&self) -> &str {
        println! {"calling borrow {}", &self.app_id};
        &self.app_id
    }
}


mod tests {
    use std::collections::HashSet;
    use crate::App;


    // cargo test --test test_borrow
    #[test]
    fn test_borrow() {
        let app1 = App {
            app_id: "meeting_minutes".into(),
            app_secret: "meeting_secret_abc".into(),
            tenant: "快乐平安".into(),
            liaison: "hedetao909".to_owned(),
            system: "subsystem code".to_owned(),// 子系统编号
        };
        let app2 = App {
            app_id: "meeting_minutes2".into(),
            app_secret: "meeting_secret_abc".into(),
            tenant: "快乐平安".into(),
            liaison: "hedetao909".to_owned(),
            system: "subsystem code".to_owned(),// 子系统编号
        };

        let v = vec![app1, app2];
        let mut app_set = HashSet::new();
        v.into_iter().for_each(|a| {
            app_set.insert(a);
        });
        // 如果不定义borrow这里就不能使用&str
        println!("{:?}", app_set.get("meeting_minutes").unwrap());
        println!("{:?}", app_set.get("meeting_minutes3").unwrap_or(&App::default()));
    }
}