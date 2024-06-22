use crate::dao::base::{Entity, EntityDao};
use chrono::prelude::*;
use futures::stream::StreamExt;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{self, IndexOptions},
    Collection, IndexModel,
};
use std::time::Duration;
// 需要引入这个trait
use serde::{Deserialize, Serialize};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao;
use crate::pb::engine::qdrant::collection;
use crate::pb::entity;
use crate::pb::svr::{
    index::{self, IndexReq, IndexResp},
    ApiError,
};
use crate::utils;
use std::result::Result;
pub const ENTITY_INDEX: &'static str = "index_entity";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexEntity {
    pub id: Option<ObjectId>,
    pub app_id: String,
    pub name: String, // 索引名称; 也是alias
    pub active: Option<String>,
    pub inactive: Option<String>,

    pub setting: index::Setting,
    pub mapping: Vec<index::MappingField>, // 设置字段以及类型
    pub configure: index::Configure,
    #[serde(with = "chrono_datetime_as_bson_datetime")] //只能支持utc
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: i64,
}

impl From<IndexReq> for IndexEntity {
    fn from(value: IndexReq) -> Self {
        Self {
            id: None,
            app_id: value.app_id,
            name: value.name,
            active: value.active,
            inactive: value.inactive,
            setting: value.setting,
            mapping: value.mapping,
            configure: value.configure,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: 0,
        }
    }
}

impl Into<IndexResp> for IndexEntity {
    fn into(self) -> IndexResp {
        let id_str = match self.id {
            Some(o) => o.to_hex(),
            None => "".to_owned(),
        };
        IndexResp {
            id: id_str,
            app_id: self.app_id,
            name: self.name,
            active: self.active,
            inactive: self.inactive,
            setting: self.setting,
            mapping: self.mapping,
            configure: self.configure,
            created_at: utils::format_chrono_utc_to_local(&self.created_at),
            updated_at: utils::format_chrono_utc_to_local(&self.updated_at),
            deleted_at: self.deleted_at,
        }
    }
}

impl Into<collection::CreateCollection> for IndexEntity {
    fn into(self) -> collection::CreateCollection {
        collection::CreateCollection {
            collection_name: self.inactive.take(),
            
        }
    }
}

impl PartialEq<IndexEntity> for IndexEntity {
    fn eq(&self, other: &IndexEntity) -> bool {
        self.name == other.name
    }
}

impl entity::Namer for IndexEntity {
    fn name(&self) -> &'static str {
        ENTITY_INDEX
    }
}

impl Entity for IndexEntity {
    fn update(&mut self, id: Option<ObjectId>, updated_at: DateTime<Utc>) {
        self.id = id;
        self.updated_at = updated_at;
    }
    fn updating_doc(&self, rhs: &Self) -> Document {
        doc! {
            "app_id": rhs.app_id.clone(),
            "active": rhs.active.clone(),
            "inactive": rhs.inactive.clone(),
            "setting": rhs.setting.clone(),  // 这里需要实现into<Bson>, 会完成自动转化
            "mapping": rhs.mapping.clone(),
            "configure": rhs.configure.clone(),
            "updated_at": rhs.updated_at,
        }
    }
}

impl IndexEntity {}
#[derive(Clone)]
pub struct IndexDao {
    collection: Collection<IndexEntity>,
}

impl EntityDao<IndexEntity> for IndexDao {
    fn new() -> Self {
        let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection(
            &MONGO_CLIENT,
            &config_file.mongo.database,
            &config_file.table.index,
        );
        IndexDao { collection: col }
    }
    fn col(&self) -> Collection<IndexEntity> {
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
                    "name":-1,"deleted_at":-1,
                })
                .options(uniqueOpt)
                .build(),
        );
        indices.push(
            IndexModel::builder()
                .keys(doc! {
                    "app_id":-1,"deleted_at":-1,
                })
                .options(opt.clone())
                .build(),
        );
        indices
    }
}

// impl IndexDao {
//     pub fn new() -> Self {
//         let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
//         let col = utils::mongo::get_collection(
//             &MONGO_CLIENT,
//             &config_file.mongo.database,
//             &config_file.table.index,
//         );
//         IndexDao { collection: col }
//     }

// }
