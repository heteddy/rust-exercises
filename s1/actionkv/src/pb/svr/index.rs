use serde::{Deserialize, Serialize, Serializer};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MappingField {
    pub name: String,
    pub field_type: String, //支持的类型
    pub is_vector: bool,
}
