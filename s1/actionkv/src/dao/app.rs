use crate::config::mongo;
// use anyhow::{Ok, Result};
use chrono::prelude::*;
use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use mongodb::bson;
use mongodb::options;
use mongodb::bson::serde_helpers::{
    chrono_datetime_as_bson_datetime, hex_string_as_object_id, serialize_hex_string_as_object_id,
};
use std::str::FromStr;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult}, //modify here
    Client,
    Collection,
};

use serde_derive::{Deserialize, Serialize};
// use serde::{Serialize, Deserialize};
use std::hash::Hasher;
use serde_json::to_string;
use tracing::info;
use crate::config::{self, mongo::MONGO_CLIENT};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppEntity {
    // serialize a hex string as an ObjectId and deserialize a hex string from an ObjectId
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    // pub id: Option<bson::oid::ObjectId>,
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub app_id: String,
    pub app_secret: String,
    // 租户名称，
    pub tenant: String,
    // 联系人
    pub liaison: String,
    //子系统名称
    pub system: String,
    // 创建时间
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    // 修改时间
    #[serde(with = "chrono_datetime_as_bson_datetime")] //使用local不行
    pub updated_at: DateTime<Utc>,
    // 删除时间
    pub deleted_at: u64,
}

impl Default for AppEntity {
    fn default() -> Self {
        // let local: DateTime<Local> = Local::now();
        AppEntity {
            id: None,
            app_id: "".into(),
            app_secret: "".into(),
            tenant: "".into(),
            liaison: "".to_owned(),
            system: "".to_owned(), // 子系统编号
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: 0,
        }
    }
}

// 需要定义字段判断哪个是唯一id
impl PartialEq<AppEntity> for AppEntity {
    fn eq(&self, other: &AppEntity) -> bool {
        self.app_id == other.app_id
    }
}

impl Eq for AppEntity {}

// 可以作为set和map的key
impl std::hash::Hash for AppEntity {
    fn hash<H: Hasher>(&self, state: &mut H)
        where
            H: Hasher,
    {
        self.app_id.hash(state)
    }
}

pub struct AppRepo {
    pub col: Collection<AppEntity>,
}

impl AppRepo {
    pub fn init(db: &str, collection: &str) -> Self {
        AppRepo {
            col: mongo::MONGO_CLIENT
                .get()
                .unwrap()
                .database(db)
                .collection(collection),
        }
    }

    pub async fn insert_app(&self, app: &AppEntity) -> mongodb::error::Result<InsertOneResult> {
        // let opt = options::InsertOneOptions::build();
        let ret = self.col.insert_one(app, None).await;
        ret
    }

    pub async fn get_app(&self, id: &String) -> Result<AppEntity, mongodb::error::Error> {
        let opt = options::FindOneOptions::builder().show_record_id(true).build();
        let ret = self.col.find_one(doc! {"_id": ObjectId::parse_str(id).unwrap()}, opt).await;
        println!("{:?}",ret);

        // ret.ok().expect("");
        if let Ok(Some(_app)) = ret {
            println!("{:?}", _app);
            Ok(_app)
        } else {
            Ok(AppEntity::default())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                // config::global_configure().await;
            })
    }

    // #[tokio::test]
    // async fn test_app() {
    //     // 使用if let 避免使用unwrap
    //     let app_repo = AppRepo::init("test","vector_app");
    //
    //     let entity = AppEntity{
    //         id: None,
    //         app_id: "new_app1".into(),
    //         app_secret: "1234".to_string(),
    //         tenant: "pib_core".into(),
    //         liaison: "hedetao909".to_owned(),
    //         system: "pib_core".to_owned(), // 子系统编号
    //         created_at: Utc::now(),
    //         updated_at: Utc::now(),
    //         deleted_at: 0,
    //     };
    //     let ret = app_repo.insert_app(&entity).await;
    //     println!("app_repo insert = {:?}",ret.unwrap().inserted_id);
    // }

    // #[tokio::test]
    // async fn test_find_app() {
    //     // 使用if let 避免使用unwrap
    //     let app_repo = AppRepo::init("test","vector_app");
    //
    //     let entity = AppEntity{
    //         id: None,
    //         app_id: "new_app1".into(),
    //         app_secret: "1234".to_string(),
    //         tenant: "pib_core".into(),
    //         liaison: "hedetao909".to_owned(),
    //         system: "pib_core".to_owned(), // 子系统编号
    //         created_at: Utc::now(),
    //         updated_at: Utc::now(),
    //         deleted_at: 0,
    //     };
    //     let ret = app_repo.insert_app(&entity).await;
    // }
}
