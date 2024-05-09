
// use crate::config::mongo;
// use chrono::prelude::*;
use futures::stream::{StreamExt, TryStreamExt};
use chrono::{DateTime, Utc};
use mongodb::bson;
use mongodb::options;
use mongodb::bson::serde_helpers::{
    bson_datetime_as_rfc3339_string,
    chrono_datetime_as_bson_datetime,
    // hex_string_as_object_id,
    // serialize_object_id_as_hex_string,
};
use mongodb::{
    bson::{doc, oid::ObjectId,Bson},
    results::{InsertOneResult, UpdateResult}, //modify here
    // Client,
    Collection,
};
// 需要引入这个trait
use serde::{Serialize, Deserialize, Serializer};
// 这个是derive 宏
use serde_derive::{Serialize as SerializeMacro, Deserialize as DeserializeMacro};
use std::result::Result;
use std::str::FromStr;
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::pb;
use serde_json::to_string;
use std::hash::Hasher;
use tracing::info;

#[derive(Debug, Clone, SerializeMacro, DeserializeMacro)]
pub struct AppEntity {
    // serialize a hex string as an ObjectId and deserialize a hex string from an ObjectId
    #[serde(
    serialize_with = "serialize_object_id_option_as_hex_string",
    rename = "_id",
    skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
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
    #[serde(with = "chrono_datetime_as_bson_datetime")]
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

impl From<pb::app::AppReq> for AppEntity {
    fn from(value: pb::app::AppReq) -> Self {
        AppEntity {
            id: None,
            app_id: value.app_id,
            app_secret: value.app_secret,
            tenant: value.tenant,
            liaison: value.liaison,
            system: value.system, // 子系统编号
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

#[derive(Clone)]
pub struct AppRepo {
    pub col: Collection<AppEntity>,
}

    impl AppRepo {
    pub fn init(db: &str, collection: &str) -> Self {
        AppRepo {
            col: MONGO_CLIENT
                .get()
                .unwrap()
                .database(db)
                .collection(collection),
        }
    }

    pub async fn insert_app(&self, app: &AppEntity) -> Result<AppEntity,mongodb::error::Error>//mongodb::error::Result<InsertOneResult> 
    {
        // let opt = options::InsertOneOptions::build();
        let ret = self.col.insert_one(app, None).await?;
        info!("dao insert app {:?}", app);
        info!("inserted id = {:?}", &ret.inserted_id);
        // let mut entity = app.into();
        let _oid = match ret.inserted_id {
            Bson::ObjectId(_id) =>{
                Some(_id)
            },
            _ => None,
        };
        let mut app2 = app.clone();
        app2.id = _oid;
        Ok(app2)
    }

    pub async fn list(&self, skip: u64, limit: i64) -> Result<Vec<AppEntity>,mongodb::error::Error> {
        let opt = options::FindOptions::builder().limit(Some(limit)).skip(Some(skip)).build();
        let mut cursor = self.col.find(None, opt).await?;
        let mut v = Vec::new();
        while let Some(doc) = cursor.next().await {
            if doc.is_ok() {
                v.push(doc.unwrap_or_default());
            }
        }
        Ok(v)
    }

    pub async fn get_app(&self, id: impl AsRef<str>) -> Result<AppEntity, mongodb::error::Error> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();
        let ret = self
            .col
            .find_one(doc! {"_id": ObjectId::parse_str(id).unwrap()}, opt)
            .await?;
        // ret.ok().expect("");
        Ok(ret.unwrap_or_default())
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

fn serialize_object_id_option_as_hex_string<S: Serializer>(
    val: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        Some(oid) => oid.to_hex().serialize(serializer),
        None => serializer.serialize_none(),
    }
}
