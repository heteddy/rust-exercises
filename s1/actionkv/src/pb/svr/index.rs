use mongodb::bson;
use serde::{Deserialize, Serialize};
// use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MappingField {
    pub name: String,
    pub field_type: String, //支持的类型
    pub is_vector: bool,
}

impl Into<bson::Bson> for MappingField {
    fn into(self) -> bson::Bson {
        bson::to_bson(&self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Setting {
    pub bert: String,
    pub server: String,
    pub preprocess: String,
}

impl Into<bson::Bson> for Setting {
    fn into(self) -> bson::Bson {
        bson::to_bson(&self).unwrap()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Configure {
    pub bert: String,
    pub server: String,
    pub preprocess: String,
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
    pub setting: Setting,
    pub mapping: Vec<MappingField>, // 设置字段以及类型
    pub configure: Configure,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IndexResp {
    pub id: String,
    pub app_id: String,
    pub name: String, // 索引名称; 也是alias
    pub active: Option<String>,
    pub inactive: Option<String>,
    pub setting: Setting,
    pub mapping: Vec<MappingField>, // 设置字段以及类型
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
