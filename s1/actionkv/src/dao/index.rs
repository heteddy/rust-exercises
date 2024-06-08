use chrono::prelude::*;
use futures::stream::StreamExt;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{self, IndexOptions},
    Collection, IndexModel,
};
use std::time::Duration;

// 需要引入这个trait
use serde::{Deserialize, Serialize};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao;
use crate::pb::entity;
use crate::pb::svr::{
    index::{self, IndexReq, IndexResp},
    ApiError,
};
use crate::utils::{self};

use std::result::Result;

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

impl PartialEq<IndexEntity> for IndexEntity {
    fn eq(&self, other: &IndexEntity) -> bool {
        self.name == other.name
    }
}

impl entity::Namer for IndexEntity {
    fn name(&self) -> &'static str {
        dao::ENTITY_INDEX
    }
}

impl IndexEntity {}

pub struct IndexDao {
    col: Collection<IndexEntity>,
}

impl IndexDao {
    pub async fn create_index() -> Result<(), ApiError> {
        let _configure = &config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection::<IndexEntity>(
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
        let o = options::CreateIndexOptions::builder()
            .max_time(Duration::from_secs(60))
            .build();
        col.create_indexes(indices, o).await?;
        Ok(())
    }

    pub fn new() -> Self {
        let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection(
            &MONGO_CLIENT,
            &config_file.mongo.database,
            &config_file.table.index,
        );
        IndexDao { col }
    }

    pub async fn list(&self, skip: u64, limit: i64) -> Result<Vec<IndexEntity>, ApiError> {
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

    pub async fn get_by_name(&self, name: impl AsRef<str>) -> Result<IndexEntity, ApiError> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();

        let ret = self.col.find_one(doc! {"name":name.as_ref()}, opt).await?;
        Ok(ret.unwrap_or_default())
    }

    pub async fn get(&self, _id: impl AsRef<str>) -> Result<IndexEntity, ApiError> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();
        let _id = ObjectId::parse_str(_id.as_ref())?;

        let ret = self.col.find_one(doc! {"name":_id}, opt).await?;
        Ok(ret.unwrap_or_default())
    }

    pub async fn update(
        &self,
        _id: impl AsRef<str>,
        mut e: IndexEntity,
    ) -> Result<IndexEntity, ApiError> {
        let opt = options::FindOneAndUpdateOptions::builder()
            .upsert(false)
            .build();
        let _id = ObjectId::parse_str(_id.as_ref())?;
        // let updated_at = Utc::now();

        let ret = self
            .col
            .find_one_and_update(
                doc! {
                    "_id":_id,
                },
                doc! {
                    "active": e.active.clone(),
                    "inactive": e.inactive.clone(),
                    "setting": e.setting.clone(),  // 这里需要实现into<Bson>, 会完成自动转化
                    "mapping": e.mapping.clone(),
                    "configure": e.configure.clone(),
                    "updated_at": e.updated_at,
                },
                opt,
            )
            .await?;
        e.id = Some(_id);
        Ok(e)
    }

    pub async fn delete(&self, _id: impl AsRef<str>) -> Result<IndexEntity, ApiError> {
        let opt = options::FindOneAndDeleteOptions::builder().build();
        let _id = ObjectId::parse_str(_id)?;
        let ret = self
            .col
            .find_one_and_delete(
                doc! {
                    "_id":_id,
                },
                opt,
            )
            .await?;
        Ok(ret.unwrap_or_default())
    }
}
