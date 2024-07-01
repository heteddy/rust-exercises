use mongodb::bson;
use serde::{Deserialize, Serialize};
// use validator::Validate;
use crate::pb::engine::qdrant::points;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MappingField {
    pub name: String,
    // pub field_type: String, //支持的类型
    #[serde(flatten)]
    pub schema: points::FieldSchema,
    pub is_vector: bool,
    pub is_index: bool,
}

impl Into<bson::Bson> for MappingField {
    fn into(self) -> bson::Bson {
        bson::to_bson(&self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Setting {
    pub replicas: u32,
    pub shards: u32,
    pub vector_size: i32, // Euclid,Cosine
    pub distance: String, //
}

/*
Distance::Cosine => "Cosine",
Distance::Euclid => "Euclid",
Distance::Dot => "Dot",
Distance::Manhattan => "Manhattan",
*/

impl Into<bson::Bson> for Setting {
    fn into(self) -> bson::Bson {
        bson::to_bson(&self).unwrap()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Configure {
    pub bert: String,
    pub server: String,
    pub preprocess: String, // 模版不需要跟index绑定
}

impl Into<bson::Bson> for Configure {
    fn into(self) -> bson::Bson {
        bson::to_bson(&self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IndexReq {
    pub app_id: String,
    pub name: String, // 索引名称; 也是alias
    pub active: Option<String>,
    pub inactive: Option<String>,
    pub settings: Setting,
    pub mappings: Vec<MappingField>, // 设置字段以及类型
    pub configure: Configure,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IndexResp {
    pub id: String,
    pub app_id: String,
    pub name: String, // 索引名称; 也是alias
    pub active: Option<String>,
    pub inactive: Option<String>,
    pub settings: Setting,
    pub mappings: Vec<MappingField>, // 设置字段以及类型
    pub configure: Configure,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: i64,
}

impl PartialEq<IndexResp> for IndexResp {
    fn eq(&self, other: &IndexResp) -> bool {
        self.name == other.name
    }
}
