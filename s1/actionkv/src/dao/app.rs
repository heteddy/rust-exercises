// use crate::config::mongo;
// use chrono::prelude::*;
use chrono::{DateTime, Local, Utc};
use futures::stream::StreamExt;
//cursor 使用
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
    options::{self, IndexOptions},
    Collection, IndexModel,
};
use std::time::Duration;
// 需要引入这个trait
use serde::{Deserialize, Serialize};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao;
use crate::pb;
use crate::pb::svr::{
    app::{AppReq, AppResp},
    ApiError,
};
use crate::utils;
use crate::utils::mongo::serialize_object_id_option_as_hex_string;
use std::hash::Hasher;
use std::result::Result;
use tracing::info;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    #[serde(with = "chrono_datetime_as_bson_datetime")] //只能支持utc
    pub created_at: DateTime<Utc>,
    // 修改时间
    // #[serde(serialize_with = "serialize_with_local_string")]
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    // 删除时间
    pub deleted_at: i64,
}

impl pb::entity::Namer for AppEntity {
    fn name(&self) -> &'static str {
        dao::ENTITY_APP
    }
}

impl Into<AppResp> for AppEntity {
    fn into(self) -> AppResp {
        let id_str = match self.id {
            Some(o) => o.to_hex(),
            None => "".to_owned(),
        };
        AppResp {
            id: id_str,
            app_id: self.app_id,
            app_secret: self.app_secret.clone(),
            tenant: self.tenant.clone(),
            liaison: self.liaison.clone(),
            system: self.system.clone(), // 子系统编号
            created_at: utils::format_chrono_utc_to_local(&self.created_at),
            updated_at: utils::format_chrono_utc_to_local(&self.updated_at),
            deleted_at: self.deleted_at,
        }
    }
}

impl From<AppReq> for AppEntity {
    fn from(value: AppReq) -> Self {
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

// impl Eq for AppEntity {}

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
    pub async fn create_index() -> Result<(), ApiError> {
        let _configure = &config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection::<AppEntity>(
            &MONGO_CLIENT,
            &_configure.mongo.database,
            &_configure.table.app,
        );

        let uniqueOpt = IndexOptions::builder()
            .unique(true)
            .background(true)
            .build();
        let opt = IndexOptions::builder()
            .unique(false)
            .background(true)
            .build();

        let mut indices = Vec::with_capacity(3);
        // note 没有指定名字，默认生成，导致问题是修改比较困难
        indices.push(
            IndexModel::builder()
                .keys(doc! {
                    "updated_at":-1,"deleted_at":-1,
                })
                .options(opt.clone())
                .build(),
        );

        indices.push(
            IndexModel::builder()
                .keys(doc! {
                    "system":1,"deleted_at":-1,
                })
                .options(opt.clone())
                .build(),
        );
        indices.push(
            IndexModel::builder()
                .keys(doc! {
                    "app_id":1,"deleted_at":-1,
                })
                .options(uniqueOpt)
                .build(),
        );
        let o = options::CreateIndexOptions::builder()
            .max_time(Duration::from_secs(60))
            .build();
        col.create_indexes(indices, o).await?;
        Ok(())
    }

    pub fn new() -> AppRepo {
        let _configure = &config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection::<AppEntity>(
            &MONGO_CLIENT,
            &_configure.mongo.database,
            &_configure.table.app,
        );
        AppRepo { col }
    }
    //mongodb::error::Result<InsertOneResult>
    pub async fn insert(&self, app: &AppEntity) -> Result<AppEntity, ApiError> {
        // let opt = options::InsertOneOptions::build();
        let ret = self.col.insert_one(app, None).await?;
        info!("dao insert app {:?}", app);
        info!("inserted id = {:?}", &ret.inserted_id);
        // let mut entity = app.into();
        let _oid = match ret.inserted_id {
            Bson::ObjectId(_id) => Some(_id),
            _ => None,
        };
        let mut app2 = app.clone();
        app2.id = _oid;
        Ok(app2)
    }

    pub async fn update_by_id(
        &self,
        id: impl AsRef<str>,
        mut app: AppEntity,
    ) -> Result<AppEntity, ApiError> {
        let opt = options::FindOneAndUpdateOptions::builder()
            .upsert(false)
            .build();
        let oid = ObjectId::parse_str(id)?;

        let updated_at = Utc::now();
        let updating = doc! {
            "$set": doc! {
                "app_id": &app.app_id,
                "app_secret": &app.app_secret,
                "tenant": &app.tenant,
                "liaison": &app.liaison,
                "system": &app.system,
                "updated_at": updated_at,
            }
        };
        let ret = self
            .col
            .find_one_and_update(doc! {"_id": oid}, updating, opt)
            .await?;

        app.updated_at = updated_at;
        app.id = Some(oid);

        Ok(app)
    }

    pub async fn list(
        &self,
        skip: u64,
        limit: i64,
    ) -> Result<Vec<AppEntity>, mongodb::error::Error> {
        let opt = options::FindOptions::builder()
            .sort(doc! {"updated_at":-1,"deleted_at":1})
            .limit(Some(limit))
            .skip(Some(skip))
            .build();
        let filters = doc! {"deleted_at":0};
        let mut cursor = self.col.find(filters, opt).await?;
        let mut v = Vec::new();
        while let Some(doc) = cursor.next().await {
            if doc.is_ok() {
                v.push(doc.unwrap_or_default());
            }
        }
        Ok(v)
    }

    pub async fn get(&self, id: impl AsRef<str>) -> Result<AppEntity, ApiError> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();

        let oid = ObjectId::parse_str(id)?;
        let ret = self.col.find_one(doc! {"_id": oid}, opt).await?;
        // ret.ok().expect("");
        Ok(ret.unwrap_or_default())
    }

    pub async fn get_by_app_id(&self, app_id: impl AsRef<str>) -> Result<AppEntity, ApiError> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();

        let ret = self
            .col
            .find_one(doc! {"app_id": app_id.as_ref()}, opt)
            .await?;
        // ret.ok().expect("");
        Ok(ret.unwrap_or_default())
    }

    pub async fn delete_by_id(&self, id: impl AsRef<str>) -> Result<AppEntity, ApiError> {
        let opt = options::FindOneAndDeleteOptions::builder()
            // .show_record_id(true)
            .build();
        let oid = ObjectId::parse_str(id)?;
        let ret = self.col.find_one_and_delete(doc! {"_id": oid}, opt).await?;
        // ret.ok().expect("");
        Ok(ret.unwrap_or_default())
    }

    pub async fn soft_delete_by_id(&self, id: impl AsRef<str>) -> Result<AppEntity, ApiError> {
        let opt = options::FindOneAndUpdateOptions::builder()
            .upsert(false)
            .build();
        let oid = ObjectId::parse_str(id)?;
        let seconds = Local::now().timestamp();
        let update = doc! { "$set": doc! { "deleted_at": bson::Bson::from(seconds)  } };
        let ret = self
            .col
            .find_one_and_update(doc! {"_id": oid}, update, opt)
            .await?;
        // ret.ok().expect("");
        Ok(ret.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {

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
