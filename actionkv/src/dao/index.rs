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
// use std::collections::Vec
// 需要引入这个trait
use serde::{Deserialize, Serialize};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao;
use crate::pb::engine::qdrant::{collection, points};
use crate::pb::entity;
use crate::pb::svr::{
    index::{self, IndexReq, IndexResp},
    ApiError,
};
use crate::utils;
use crate::utils::mongo::serialize_object_id_option_as_hex_string;
use std::result::Result;
use tracing::{info, instrument};
pub const ENTITY_INDEX: &'static str = "index_entity";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexEntity {
    #[serde(
        serialize_with = "serialize_object_id_option_as_hex_string",
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
    pub app_id: String,
    pub name: String, // 索引名称; 也是alias
    pub active: Option<String>,
    pub inactive: Option<String>,
    pub settings: index::Setting,
    pub mappings: Vec<index::MappingField>, // 设置字段以及类型
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
            settings: value.settings,
            mappings: value.mappings,
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
            settings: self.settings,
            mappings: self.mappings,
            configure: self.configure,
            created_at: utils::format_chrono_utc_to_local(&self.created_at),
            updated_at: utils::format_chrono_utc_to_local(&self.updated_at),
            deleted_at: self.deleted_at,
        }
    }
}

impl Into<collection::ChangeAliases> for IndexEntity {
    fn into(self) -> collection::ChangeAliases {
        let mut vec = Vec::with_capacity(2);
        let del_alias = collection::DeleteAlias {
            alias_name: self.name.clone(),
        };

        let op1 = collection::AliasOperations {
            action: Some(collection::alias_operations::Action::DeleteAlias(del_alias)),
        };
        vec.push(op1);
        let add_alias = collection::CreateAlias {
            collection_name: self.inactive.unwrap().clone(),
            alias_name: self.name.clone(),
        };
        let op2 = collection::AliasOperations {
            action: Some(collection::alias_operations::Action::CreateAlias(add_alias)),
        };
        vec.push(op2);
        let req = collection::ChangeAliases {
            actions: vec,
            timeout: None,
        };
        req
    }
}

impl Into<collection::CreateCollection> for IndexEntity {
    fn into(self) -> collection::CreateCollection {
        let oc = collection::OptimizersConfigDiff {
            indexing_threshold: Some(1000),
            ..Default::default()
        };
        let vc = collection::VectorParams {
            size: self.settings.vector_size as u64,
            distance: collection::Distance::Euclid.as_str_name().to_owned(),
            on_disk: Some(true),
            ..Default::default()
        };

        let cc = collection::CreateCollection {
            collection_name: self.inactive.unwrap(),
            on_disk_payload: Some(true),
            optimizers_config: Some(oc),
            vectors: Some(vc),
            shard_number: Some(self.settings.shards),
            replication_factor: Some(self.settings.replicas),
            ..Default::default()
        };
        // cc.optimizers_config =
        cc
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
        let d = doc! {
            "app_id": rhs.app_id.clone(),
            "active": rhs.active.clone(),
            "inactive": rhs.inactive.clone(),
            "setting": rhs.settings.clone(),  // 这里需要实现into<Bson>, 会完成自动转化
            "mapping": rhs.mappings.clone(),
            "configure": rhs.configure.clone(),
            "updated_at": rhs.updated_at,
        };
        info!("generating doc {:?}", d);
        d
    }
}

impl IndexEntity {
    // pub struct MappingField {
    //     pub name: String,
    //     pub field_schema: points::FieldSchema,
    //     pub is_vector: bool,
    //     pub is_index: bool,
    // }
    pub fn to_field_index_collection(&self) -> Vec<points::CreateFieldIndexCollection> {
        let mut rets = Vec::with_capacity(self.mappings.len());

        for f in &self.mappings {
            if f.is_index {
                let inactive = self.inactive.clone();
                rets.push(points::CreateFieldIndexCollection {
                    collection_name: inactive.unwrap(),
                    wait: Some(false),
                    field_name: f.name.clone(),
                    schema: Some(f.schema.clone()),
                    ..Default::default()
                });
            }
        }
        rets
    }
}
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
