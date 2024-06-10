// use crate::config::mongo;
// use chrono::prelude::*;
use chrono::{DateTime, Local, Utc};
use futures::stream::StreamExt;
//cursor 使用
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, Document},
    options::{self, IndexOptions},
    Collection, IndexModel,
};
use std::time::Duration;
// 需要引入这个trait
use serde::{Deserialize, Serialize};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
pub const ENTITY_APP: &'static str = "app_entity";
use crate::dao::base::{Entity, EntityDao};
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
        ENTITY_APP
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
            app_secret: self.app_secret,
            tenant: self.tenant,
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

impl Entity for AppEntity {
    fn update(&mut self, id: Option<ObjectId>, updated_at: DateTime<Utc>) {
        self.id = id;
        self.updated_at = updated_at;
    }
    fn updating_doc(&self, rhs: &Self) -> Document {
        doc! {
            "app_id": rhs.app_id.clone(),
            "app_secret": rhs.app_secret.clone(),
            "tenant": rhs.tenant.clone(),
            "liaison": rhs.liaison.clone(),
            "system": rhs.system.clone(),
            "updated_at": rhs.updated_at,
        }
    }
}

#[derive(Clone)]
pub struct AppDao {
    collection: Collection<AppEntity>,
}

impl EntityDao<AppEntity> for AppDao {
    fn col(&self) -> Collection<AppEntity> {
        self.collection.clone()
    }
    fn indices(&self) -> Vec<IndexModel> {
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
        indices
    }
}

impl AppDao {
    pub fn new() -> AppDao {
        let _configure = &config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection::<AppEntity>(
            &MONGO_CLIENT,
            &_configure.mongo.database,
            &_configure.table.app,
        );
        AppDao { collection: col }
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
}
