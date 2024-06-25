use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VectorParams {
//     /// Size of the vectors
//     pub size: u64,
//     /// Distance function used for comparing vectors
//     pub distance: i32,
//     /// Configuration of vector HNSW graph. If omitted - the collection configuration will be used
//     pub hnsw_config: Option<HnswConfigDiff>,
//     /// Configuration of vector quantization config. If omitted - the collection configuration will be used
//     pub quantization_config: Option<QuantizationConfig>,
//     /// If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM.
//     pub on_disk: Option<bool>,
//     /// Data type of the vectors
//     pub datatype: Option<i32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VectorParamsDiff {
//     /// Update params for HNSW index. If empty object - it will be unset
//     pub hnsw_config: Option<HnswConfigDiff>,
//     /// Update quantization params. If none - it is left unchanged.
//     pub quantization_config: Option<QuantizationConfigDiff>,
//     /// If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM.
//     pub on_disk: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VectorParamsMap {
//     pub map: ::std::collections::HashMap<String, VectorParams>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VectorParamsDiffMap {
//     pub map: ::std::collections::HashMap<String, VectorParamsDiff>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VectorsConfig {
//     pub config: Option<vectors_config::Config>,
// }
// /// Nested message and enum types in `VectorsConfig`.
// pub mod vectors_config {
//     use super::*;
//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum Config {
//         Params(super::VectorParams),
//         ParamsMap(super::VectorParamsMap),
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VectorsConfigDiff {
//     pub config: Option<vectors_config_diff::Config>,
// }
// /// Nested message and enum types in `VectorsConfigDiff`.
// pub mod vectors_config_diff {
//     use super::*;
//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum Config {
//         Params(super::VectorParamsDiff),
//         ParamsMap(super::VectorParamsDiffMap),
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SparseVectorParams {
//     /// Configuration of sparse index
//     pub index: Option<SparseIndexConfig>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SparseVectorConfig {
//     pub map: ::std::collections::HashMap<String, SparseVectorParams>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct GetCollectionInfoRequest {
//     /// Name of the collection
//     pub collection_name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionExistsRequest {
//     pub collection_name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionExists {
//     pub exists: bool,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionExistsResponse {
//     pub result: Option<CollectionExists>,
//     /// Time spent to process
//     pub time: f64,
// }
// impl IntoResponse for CollectionExistsResponse {
//     fn into_response(self) -> Response {
//         Json(self).into_response()
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ListCollectionsRequest {}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionDescription {
//     /// Name of the collection
//     pub name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct GetCollectionInfoResponse {
//     pub result: Option<CollectionInfo>,
//     /// Time spent to process
//     pub time: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ListCollectionsResponse {
//     pub collections: Vec<CollectionDescription>,
//     /// Time spent to process
//     pub time: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct OptimizerStatus {
//     pub ok: bool,
//     pub error: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct HnswConfigDiff {
//     ///
//     /// Number of edges per node in the index graph. Larger the value - more accurate the search, more space required.
//     pub m: Option<u64>,
//     ///
//     /// Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build the index.
//     pub ef_construct: Option<u64>,
//     ///
//     /// Minimal size (in KiloBytes) of vectors for additional payload-based indexing.
//     /// If the payload chunk is smaller than `full_scan_threshold` additional indexing won't be used -
//     /// in this case full-scan search should be preferred by query planner and additional indexing is not required.
//     /// Note: 1 Kb = 1 vector of size 256
//     pub full_scan_threshold: Option<u64>,
//     ///
//     /// Number of parallel threads used for background index building.
//     /// If 0 - automatically select from 8 to 16.
//     /// Best to keep between 8 and 16 to prevent likelihood of building broken/inefficient HNSW graphs.
//     /// On small CPUs, less threads are used.
//     pub max_indexing_threads: Option<u64>,
//     ///
//     /// Store HNSW index on disk. If set to false, the index will be stored in RAM.
//     pub on_disk: Option<bool>,
//     ///
//     /// Number of additional payload-aware links per node in the index graph. If not set - regular M parameter will be used.
//     pub payload_m: Option<u64>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SparseIndexConfig {
//     ///
//     /// Prefer a full scan search upto (excluding) this number of vectors.
//     /// Note: this is number of vectors, not KiloBytes.
//     pub full_scan_threshold: Option<u64>,
//     ///
//     /// Store inverted index on disk. If set to false, the index will be stored in RAM.
//     pub on_disk: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct WalConfigDiff {
//     /// Size of a single WAL block file
//     pub wal_capacity_mb: Option<u64>,
//     /// Number of segments to create in advance
//     pub wal_segments_ahead: Option<u64>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct OptimizersConfigDiff {
//     ///
//     /// The minimal fraction of deleted vectors in a segment, required to perform segment optimization
//     pub deleted_threshold: Option<f64>,
//     ///
//     /// The minimal number of vectors in a segment, required to perform segment optimization
//     pub vacuum_min_vector_number: Option<u64>,
//     ///
//     /// Target amount of segments the optimizer will try to keep.
//     /// Real amount of segments may vary depending on multiple parameters:
//     ///
//     /// - Amount of stored points.
//     /// - Current write RPS.
//     ///
//     /// It is recommended to select the default number of segments as a factor of the number of search threads,
//     /// so that each segment would be handled evenly by one of the threads.
//     pub default_segment_number: Option<u64>,
//     ///
//     /// Do not create segments larger this size (in kilobytes).
//     /// Large segments might require disproportionately long indexation times,
//     /// therefore it makes sense to limit the size of segments.
//     ///
//     /// If indexing speed is more important - make this parameter lower.
//     /// If search speed is more important - make this parameter higher.
//     /// Note: 1Kb = 1 vector of size 256
//     /// If not set, will be automatically selected considering the number of available CPUs.
//     pub max_segment_size: Option<u64>,
//     ///
//     /// Maximum size (in kilobytes) of vectors to store in-memory per segment.
//     /// Segments larger than this threshold will be stored as read-only memmaped file.
//     ///
//     /// Memmap storage is disabled by default, to enable it, set this threshold to a reasonable value.
//     ///
//     /// To disable memmap storage, set this to `0`.
//     ///
//     /// Note: 1Kb = 1 vector of size 256
//     pub memmap_threshold: Option<u64>,
//     ///
//     /// Maximum size (in kilobytes) of vectors allowed for plain index, exceeding this threshold will enable vector indexing
//     ///
//     /// Default value is 20,000, based on <<https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md>.>
//     ///
//     /// To disable vector indexing, set to `0`.
//     ///
//     /// Note: 1kB = 1 vector of size 256.
//     pub indexing_threshold: Option<u64>,
//     ///
//     /// Interval between forced flushes.
//     pub flush_interval_sec: Option<u64>,
//     ///
//     /// Max number of threads (jobs) for running optimizations per shard.
//     /// Note: each optimization job will also use `max_indexing_threads` threads by itself for index building.
//     /// If null - have no limit and choose dynamically to saturate CPU.
//     /// If 0 - no optimization threads, optimizations will be disabled.
//     pub max_optimization_threads: Option<u64>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ScalarQuantization {
//     /// Type of quantization
//     pub r#type: i32,
//     /// Number of bits to use for quantization
//     pub quantile: Option<f32>,
//     /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
//     pub always_ram: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ProductQuantization {
//     /// Compression ratio
//     pub compression: i32,
//     /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
//     pub always_ram: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct BinaryQuantization {
//     /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
//     pub always_ram: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct QuantizationConfig {
//     pub quantization: Option<quantization_config::Quantization>,
// }
// /// Nested message and enum types in `QuantizationConfig`.
// pub mod quantization_config {
//     use super::*;
//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum Quantization {
//         Scalar(super::ScalarQuantization),
//         Product(super::ProductQuantization),
//         Binary(super::BinaryQuantization),
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Disabled {}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct QuantizationConfigDiff {
//     pub quantization: Option<quantization_config_diff::Quantization>,
// }
// /// Nested message and enum types in `QuantizationConfigDiff`.
// pub mod quantization_config_diff {
//     use super::*;
//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum Quantization {
//         Scalar(super::ScalarQuantization),

//         Product(super::ProductQuantization),

//         Disabled(super::Disabled),

//         Binary(super::BinaryQuantization),
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CreateCollection {
//     /// Name of the collection
//     pub collection_name: String,
//     /// Configuration of vector index
//     pub hnsw_config: Option<HnswConfigDiff>,
//     /// Configuration of the Write-Ahead-Log
//     pub wal_config: Option<WalConfigDiff>,
//     /// Configuration of the optimizers
//     pub optimizers_config: Option<OptimizersConfigDiff>,
//     /// Number of shards in the collection, default is 1 for standalone, otherwise equal to the number of nodes. Minimum is 1
//     pub shard_number: Option<u32>,
//     /// If true - point's payload will not be stored in memory
//     pub on_disk_payload: Option<bool>,
//     /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
//     pub timeout: Option<u64>,
//     /// Configuration for vectors
//     pub vectors_config: Option<VectorsConfig>,
//     /// Number of replicas of each shard that network tries to maintain, default = 1
//     pub replication_factor: Option<u32>,
//     /// How many replicas should apply the operation for us to consider it successful, default = 1
//     pub write_consistency_factor: Option<u32>,
//     /// Specify name of the other collection to copy data from
//     pub init_from_collection: Option<String>,
//     /// Quantization configuration of vector
//     pub quantization_config: Option<QuantizationConfig>,
//     /// Sharding method
//     pub sharding_method: Option<i32>,
//     /// Configuration for sparse vectors
//     pub sparse_vectors_config: Option<SparseVectorConfig>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UpdateCollection {
//     /// Name of the collection
//     pub collection_name: String,
//     /// New configuration parameters for the collection. This operation is blocking, it will only proceed once all current optimizations are complete
//     pub optimizers_config: Option<OptimizersConfigDiff>,
//     /// Wait timeout for operation commit in seconds if blocking, if not specified - default value will be supplied
//     pub timeout: Option<u64>,
//     /// New configuration parameters for the collection
//     pub params: Option<CollectionParamsDiff>,
//     /// New HNSW parameters for the collection index
//     pub hnsw_config: Option<HnswConfigDiff>,
//     /// New vector parameters
//     pub vectors_config: Option<VectorsConfigDiff>,
//     /// Quantization configuration of vector
//     pub quantization_config: Option<QuantizationConfigDiff>,
//     /// New sparse vector parameters
//     pub sparse_vectors_config: Option<SparseVectorConfig>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DeleteCollection {
//     /// Name of the collection
//     pub collection_name: String,
//     /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
//     pub timeout: Option<u64>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionOperationResponse {
//     /// if operation made changes
//     pub result: bool,
//     /// Time spent to process
//     pub time: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionParams {
//     /// Number of shards in collection
//     pub shard_number: u32,
//     /// If true - point's payload will not be stored in memory
//     pub on_disk_payload: bool,
//     /// Configuration for vectors
//     pub vectors_config: Option<VectorsConfig>,
//     /// Number of replicas of each shard that network tries to maintain
//     pub replication_factor: Option<u32>,
//     /// How many replicas should apply the operation for us to consider it successful
//     pub write_consistency_factor: Option<u32>,
//     /// Fan-out every read request to these many additional remote nodes (and return first available response)
//     pub read_fan_out_factor: Option<u32>,
//     /// Sharding method
//     pub sharding_method: Option<i32>,
//     /// Configuration for sparse vectors
//     pub sparse_vectors_config: Option<SparseVectorConfig>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionParamsDiff {
//     /// Number of replicas of each shard that network tries to maintain
//     pub replication_factor: Option<u32>,
//     /// How many replicas should apply the operation for us to consider it successful
//     pub write_consistency_factor: Option<u32>,
//     /// If true - point's payload will not be stored in memory
//     pub on_disk_payload: Option<bool>,
//     /// Fan-out every read request to these many additional remote nodes (and return first available response)
//     pub read_fan_out_factor: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionConfig {
//     /// Collection parameters
//     pub params: Option<CollectionParams>,
//     /// Configuration of vector index
//     pub hnsw_config: Option<HnswConfigDiff>,
//     /// Configuration of the optimizers
//     pub optimizer_config: Option<OptimizersConfigDiff>,
//     /// Configuration of the Write-Ahead-Log
//     pub wal_config: Option<WalConfigDiff>,
//     /// Configuration of the vector quantization
//     pub quantization_config: Option<QuantizationConfig>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextIndexParams {
    /// Tokenizer type
    pub tokenizer: i32,
    /// If true - all tokens will be lowercase
    pub lowercase: Option<bool>,
    /// Minimal token length
    pub min_token_len: Option<u64>,
    /// Maximal token length
    pub max_token_len: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerIndexParams {
    /// If true - support direct lookups.
    pub lookup: bool,
    /// If true - support ranges filters.
    pub range: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadIndexParams {
    #[serde(flatten)]
    pub index_params: Option<payload_index_params::IndexParams>,
}
/// Nested message and enum types in `PayloadIndexParams`.
pub mod payload_index_params {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum IndexParams {
        /// Parameters for text index
        TextIndexParams(super::TextIndexParams),
        /// Parameters for integer index
        IntegerIndexParams(super::IntegerIndexParams),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadSchemaInfo {
    /// Field data type
    pub data_type: i32,
    /// Field index parameters
    pub params: Option<PayloadIndexParams>,
    /// Number of points indexed within this field indexed
    pub points: Option<u64>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionInfo {
//     /// operating condition of the collection
//     pub status: i32,
//     /// status of collection optimizers
//     pub optimizer_status: Option<OptimizerStatus>,
//     /// Approximate number of vectors in the collection
//     pub vectors_count: Option<u64>,
//     /// Number of independent segments
//     pub segments_count: u64,
//     /// Configuration
//     pub config: Option<CollectionConfig>,
//     /// Collection data types
//     pub payload_schema: ::std::collections::HashMap<String, PayloadSchemaInfo>,
//     /// Approximate number of points in the collection
//     pub points_count: Option<u64>,
//     /// Approximate number of indexed vectors in the collection.
//     pub indexed_vectors_count: Option<u64>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ChangeAliases {
//     /// List of actions
//     pub actions: Vec<AliasOperations>,
//     /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
//     pub timeout: Option<u64>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AliasOperations {
//     pub action: Option<alias_operations::Action>,
// }
// /// Nested message and enum types in `AliasOperations`.
// pub mod alias_operations {
//     use super::*;
//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum Action {
//         CreateAlias(super::CreateAlias),

//         RenameAlias(super::RenameAlias),

//         DeleteAlias(super::DeleteAlias),
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CreateAlias {
//     /// Name of the collection
//     pub collection_name: String,
//     /// New name of the alias
//     pub alias_name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct RenameAlias {
//     /// Name of the alias to rename
//     pub old_alias_name: String,
//     /// Name of the alias
//     pub new_alias_name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DeleteAlias {
//     /// Name of the alias
//     pub alias_name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ListAliasesRequest {}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ListCollectionAliasesRequest {
//     /// Name of the collection
//     pub collection_name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AliasDescription {
//     /// Name of the alias
//     pub alias_name: String,
//     /// Name of the collection
//     pub collection_name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ListAliasesResponse {
//     pub aliases: Vec<AliasDescription>,
//     /// Time spent to process
//     pub time: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionClusterInfoRequest {
//     /// Name of the collection
//     pub collection_name: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardKey {
    pub key: Option<shard_key::Key>,
}
/// Nested message and enum types in `ShardKey`.
pub mod shard_key {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Key {
        /// String key
        Keyword(String),
        /// Number key
        Number(u64),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalShardInfo {
    /// Local shard id
    pub shard_id: u32,
    /// Number of points in the shard
    pub points_count: u64,
    /// Is replica active
    pub state: i32,
    /// User-defined shard key
    pub shard_key: Option<ShardKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteShardInfo {
    /// Local shard id
    pub shard_id: u32,
    /// Remote peer id
    pub peer_id: u64,
    /// Is replica active
    pub state: i32,
    /// User-defined shard key
    pub shard_key: Option<ShardKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardTransferInfo {
    /// Local shard id
    pub shard_id: u32,
    pub from: u64,
    pub to: u64,
    /// If `true` transfer is a synchronization of a replicas; If `false` transfer is a moving of a shard from one peer to another
    pub sync: bool,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CollectionClusterInfoResponse {
//     /// ID of this peer
//     pub peer_id: u64,
//     /// Total number of shards
//     pub shard_count: u64,
//     /// Local shards
//     pub local_shards: Vec<LocalShardInfo>,
//     /// Remote shards
//     pub remote_shards: Vec<RemoteShardInfo>,
//     /// Shard transfers
//     pub shard_transfers: Vec<ShardTransferInfo>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveShard {
    /// Local shard id
    pub shard_id: u32,

    pub from_peer_id: u64,

    pub to_peer_id: u64,

    pub method: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortShardTransfer {
    /// Local shard id
    pub shard_id: u32,

    pub from_peer_id: u64,

    pub to_peer_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestartTransfer {
    /// Local shard id
    pub shard_id: u32,
    pub from_peer_id: u64,
    pub to_peer_id: u64,
    pub method: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replica {
    pub shard_id: u32,
    pub peer_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShardKey {
    /// User-defined shard key
    pub shard_key: Option<ShardKey>,
    /// Number of shards to create per shard key
    pub shards_number: Option<u32>,
    /// Number of replicas of each shard to create
    pub replication_factor: Option<u32>,
    /// List of peer ids, allowed to create shards. If empty - all peers are allowed
    pub placement: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteShardKey {
    /// Shard key to delete
    pub shard_key: Option<ShardKey>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UpdateCollectionClusterSetupRequest {
//     /// Name of the collection
//     pub collection_name: String,
//     /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
//     pub timeout: Option<u64>,
//     pub operation: Option<update_collection_cluster_setup_request::Operation>,
// }
// /// Nested message and enum types in `UpdateCollectionClusterSetupRequest`.
// pub mod update_collection_cluster_setup_request {
//     use super::*;
//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum Operation {
//         MoveShard(super::MoveShard),
//         ReplicateShard(super::MoveShard),
//         AbortTransfer(super::AbortShardTransfer),
//         DropReplica(super::Replica),
//         CreateShardKey(super::CreateShardKey),
//         DeleteShardKey(super::DeleteShardKey),
//         RestartTransfer(super::RestartTransfer),
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UpdateCollectionClusterSetupResponse {
//     pub result: bool,
// }

// impl IntoResponse for UpdateCollectionClusterSetupResponse {
//     fn into_response(self) -> Response {
//         Json(self).into_response()
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Datatype {
    Default = 0,
    Float32 = 1,
    Uint8 = 2,
}
impl Datatype {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Datatype::Default => "Default",
            Datatype::Float32 => "Float32",
            Datatype::Uint8 => "Uint8",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "Default" => Some(Self::Default),
            "Float32" => Some(Self::Float32),
            "Uint8" => Some(Self::Uint8),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum PayloadSchemaType {
    UnknownType = 0,
    Keyword = 1,
    Integer = 2,
    Float = 3,
    Geo = 4,
    Text = 5,
    Bool = 6,
    Datetime = 7,
}
impl PayloadSchemaType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PayloadSchemaType::UnknownType => "UnknownType",
            PayloadSchemaType::Keyword => "Keyword",
            PayloadSchemaType::Integer => "Integer",
            PayloadSchemaType::Float => "Float",
            PayloadSchemaType::Geo => "Geo",
            PayloadSchemaType::Text => "Text",
            PayloadSchemaType::Bool => "Bool",
            PayloadSchemaType::Datetime => "Datetime",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "UnknownType" => Some(Self::UnknownType),
            "Keyword" => Some(Self::Keyword),
            "Integer" => Some(Self::Integer),
            "Float" => Some(Self::Float),
            "Geo" => Some(Self::Geo),
            "Text" => Some(Self::Text),
            "Bool" => Some(Self::Bool),
            "Datetime" => Some(Self::Datetime),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum QuantizationType {
    UnknownQuantization = 0,
    Int8 = 1,
}
impl QuantizationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QuantizationType::UnknownQuantization => "UnknownQuantization",
            QuantizationType::Int8 => "Int8",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "UnknownQuantization" => Some(Self::UnknownQuantization),
            "Int8" => Some(Self::Int8),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CompressionRatio {
    X4 = 0,
    X8 = 1,
    X16 = 2,
    X32 = 3,
    X64 = 4,
}
impl CompressionRatio {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CompressionRatio::X4 => "x4",
            CompressionRatio::X8 => "x8",
            CompressionRatio::X16 => "x16",
            CompressionRatio::X32 => "x32",
            CompressionRatio::X64 => "x64",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "x4" => Some(Self::X4),
            "x8" => Some(Self::X8),
            "x16" => Some(Self::X16),
            "x32" => Some(Self::X32),
            "x64" => Some(Self::X64),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ShardingMethod {
    /// Auto-sharding based on record ids
    Auto = 0,
    /// Shard by user-defined key
    Custom = 1,
}
impl ShardingMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShardingMethod::Auto => "Auto",
            ShardingMethod::Custom => "Custom",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "Auto" => Some(Self::Auto),
            "Custom" => Some(Self::Custom),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum TokenizerType {
    Unknown = 0,
    Prefix = 1,
    Whitespace = 2,
    Word = 3,
    Multilingual = 4,
}
impl TokenizerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TokenizerType::Unknown => "Unknown",
            TokenizerType::Prefix => "Prefix",
            TokenizerType::Whitespace => "Whitespace",
            TokenizerType::Word => "Word",
            TokenizerType::Multilingual => "Multilingual",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "Unknown" => Some(Self::Unknown),
            "Prefix" => Some(Self::Prefix),
            "Whitespace" => Some(Self::Whitespace),
            "Word" => Some(Self::Word),
            "Multilingual" => Some(Self::Multilingual),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ReplicaState {
    /// Active and sound
    Active = 0,
    /// Failed for some reason
    Dead = 1,
    /// The shard is partially loaded and is currently receiving data from other shards
    Partial = 2,
    /// Collection is being created
    Initializing = 3,
    /// A shard which receives data, but is not used for search; Useful for backup shards
    Listener = 4,
    /// Deprecated: snapshot shard transfer is in progress; Updates should not be sent to (and are ignored by) the shard
    PartialSnapshot = 5,
    /// Shard is undergoing recovered by an external node; Normally rejects updates, accepts updates if force is true
    Recovery = 6,
}
impl ReplicaState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReplicaState::Active => "Active",
            ReplicaState::Dead => "Dead",
            ReplicaState::Partial => "Partial",
            ReplicaState::Initializing => "Initializing",
            ReplicaState::Listener => "Listener",
            ReplicaState::PartialSnapshot => "PartialSnapshot",
            ReplicaState::Recovery => "Recovery",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "Active" => Some(Self::Active),
            "Dead" => Some(Self::Dead),
            "Partial" => Some(Self::Partial),
            "Initializing" => Some(Self::Initializing),
            "Listener" => Some(Self::Listener),
            "PartialSnapshot" => Some(Self::PartialSnapshot),
            "Recovery" => Some(Self::Recovery),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ShardTransferMethod {
    /// Stream shard records in batches
    StreamRecords = 0,
    /// Snapshot the shard and recover it on the target peer
    Snapshot = 1,
    /// Resolve WAL delta between peers and transfer the difference
    WalDelta = 2,
}
impl ShardTransferMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShardTransferMethod::StreamRecords => "StreamRecords",
            ShardTransferMethod::Snapshot => "Snapshot",
            ShardTransferMethod::WalDelta => "WalDelta",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "StreamRecords" => Some(Self::StreamRecords),
            "Snapshot" => Some(Self::Snapshot),
            "WalDelta" => Some(Self::WalDelta),
            _ => None,
        }
    }
}

/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
///
/// The JSON representation for `Struct` is a JSON object.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    /// Unordered map of dynamically typed values.
    pub fields: ::std::collections::HashMap<String, Value>,
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of those
/// variants, absence of any variant indicates an error.
///
/// The JSON representation for `Value` is a JSON value.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    /// The kind of value.
    pub kind: Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Kind {
        /// Represents a null value.
        NullValue(i32),
        /// Represents a double value.
        DoubleValue(f64),
        /// Represents an integer value
        IntegerValue(i64),
        /// Represents a string value.
        StringValue(String),
        /// Represents a boolean value.
        BoolValue(bool),
        /// Represents a structured value.
        StructValue(super::Struct),
        /// Represents a repeated `Value`.
        ListValue(super::ListValue),
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is a JSON array.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListValue {
    /// Repeated field of dynamically typed values.
    pub values: Vec<Value>,
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.
///
///   The JSON representation for `NullValue` is JSON `null`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum NullValue {
    /// Null value.
    NullValue = 0,
}
impl NullValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NullValue::NullValue => "NULL_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "NULL_VALUE" => Some(Self::NullValue),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOrdering {
    /// Write ordering guarantees
    pub r#type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadConsistency {
    pub value: Option<read_consistency::Value>,
}
/// Nested message and enum types in `ReadConsistency`.
pub mod read_consistency {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Value {
        /// Common read consistency configurations
        Type(i32),
        /// Send request to a specified number of nodes, and return points which are present on all of them
        Factor(u64),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointId {
    pub point_id_options: Option<point_id::PointIdOptions>,
}
/// Nested message and enum types in `PointId`.
pub mod point_id {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum PointIdOptions {
        /// Numerical ID of the point
        Num(u64),
        /// UUID
        Uuid(String),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseIndices {
    pub data: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    pub data: Vec<f32>,

    pub indices: Option<SparseIndices>,
}
/// ---------------------------------------------
/// ----------------- ShardKeySelector ----------
/// ---------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardKeySelector {
    /// List of shard keys which should be used in the request
    pub shard_keys: Vec<ShardKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertPoints {
    /// name of the collection
    pub collection_name: String,
    /// Wait until the changes have been applied?
    pub wait: Option<bool>,
    pub points: Vec<PointStruct>,
    /// Write ordering guarantees
    pub ordering: Option<WriteOrdering>,
    /// Option for custom sharding to specify used shard keys
    pub shard_key_selector: Option<ShardKeySelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePoints {
    /// name of the collection
    pub collection_name: String,
    /// Wait until the changes have been applied?
    pub wait: Option<bool>,
    /// Affected points
    // pub points: Option<PointsSelector>,
    /// Write ordering guarantees
    pub ordering: Option<WriteOrdering>,
    /// Option for custom sharding to specify used shard keys
    pub shard_key_selector: Option<ShardKeySelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPoints {
    /// name of the collection
    pub collection_name: String,
    /// List of points to retrieve
    pub ids: Vec<PointId>,
    /// Options for specifying which payload to include or not
    pub with_payload: Option<WithPayloadSelector>,
    /// Options for specifying which vectors to include into response
    pub with_vectors: Option<WithVectorsSelector>,
    /// Options for specifying read consistency guarantees
    pub read_consistency: Option<ReadConsistency>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub shard_key_selector: Option<ShardKeySelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePointVectors {
    /// name of the collection
    pub collection_name: String,
    /// Wait until the changes have been applied?
    pub wait: Option<bool>,
    /// List of points and vectors to update
    pub points: Vec<PointVectors>,
    /// Write ordering guarantees
    pub ordering: Option<WriteOrdering>,
    /// Option for custom sharding to specify used shard keys
    pub shard_key_selector: Option<ShardKeySelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointVectors {
    /// ID to update vectors for
    pub id: Option<PointId>,
    /// Named vectors to update, leave others intact
    pub vectors: Option<Vectors>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFieldIndexCollection {
    /// name of the collection
    pub collection_name: String,
    /// Wait until the changes have been applied?
    pub wait: Option<bool>,
    /// Field name to index
    pub field_name: String,
    /// Field type.
    // pub field_type: Option<String>, // teddy 修改为
    /// field schema； teddy added
    pub field_schema: Option<FieldSchema>,
    /// Payload index params.
    // pub field_index_params: Option<PayloadIndexParams>,
    /// Write ordering guarantees
    pub ordering: Option<WriteOrdering>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum FieldSchema {
    //todo: teddy defined
    Keyword(KeywordIndex),
    Text(TextIndex),
    Integer(IntegerIndex),
    Float(FloatIndex),
    Bool(BoolIndex),
    Datetime(DatetimeIndex),
    Geo(GeoIndex),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordIndex {
    #[serde(rename = "type")]
    pub field_type: String,
}
/*
Available tokenizers are:

word - splits the string into words, separated by spaces, punctuation marks, and special characters.
whitespace - splits the string into words, separated by spaces.
prefix - splits the string into words, separated by spaces, punctuation marks, and special characters, and then creates a prefix index for each word. For example: hello will be indexed as h, he, hel, hell, hello.
multilingual - special type of tokenizer based on charabia package.
It allows proper tokenization and lemmatization for multiple languages,
including those with non-latin alphabets and non-space delimiters.
See charabia documentation for full list of supported languages supported normalization options.
In the default build configuration, qdrant does not include support for all languages, due to the increasing size of the resulting binary. Chinese, Japanese and Korean languages are not enabled by default,
but can be enabled by building qdrant from source with --features multiling-chinese,multiling-japanese,multiling-korean flags.
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextIndex {
    #[serde(rename = "type")]
    pub field_type: String,
    pub tokenizer: String,
    pub min_token_len: i32,
    pub max_token_len: i32,
    pub lowercase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerIndex {
    #[serde(rename = "type")]
    pub field_type: String, //integer
    pub lookup: bool, // 支持完全匹配
    pub range: bool,  // 支持范围查找
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatIndex {
    #[serde(rename = "type")]
    pub field_type: String, //float
    pub lookup: bool, // 支持完全匹配
    pub range: bool,  // 支持范围查找
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolIndex {
    #[serde(rename = "type")]
    pub field_type: String, //bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoIndex {
    #[serde(rename = "type")]
    pub field_type: String, //geo
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatetimeIndex {
    #[serde(rename = "type")]
    pub field_type: String, //geo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFieldIndexCollection {
    /// name of the collection
    pub collection_name: String,
    /// Wait until the changes have been applied?
    pub wait: Option<bool>,
    /// Field name to delete
    pub field_name: String,
    /// Write ordering guarantees
    pub ordering: Option<WriteOrdering>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetPayloadPoints {
    /// name of the collection
    pub collection_name: String,
    /// Wait until the changes have been applied?
    pub wait: Option<bool>,
    /// New payload values
    pub payload: ::std::collections::HashMap<String, Value>,
    /// Affected points
    // pub points_selector: Option<PointsSelector>,
    /// Write ordering guarantees
    pub ordering: Option<WriteOrdering>,
    /// Option for custom sharding to specify used shard keys
    pub shard_key_selector: Option<ShardKeySelector>,
    /// Option for indicate property of payload
    pub key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadIncludeSelector {
    /// List of payload keys to include into result
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadExcludeSelector {
    /// List of payload keys to exclude from the result
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithPayloadSelector {
    pub selector_options: Option<with_payload_selector::SelectorOptions>,
}
/// Nested message and enum types in `WithPayloadSelector`.
pub mod with_payload_selector {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SelectorOptions {
        /// If `true` - return all payload, if `false` - none
        Enable(bool),
        Include(super::PayloadIncludeSelector),
        Exclude(super::PayloadExcludeSelector),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedVectors {
    pub vectors: ::std::collections::HashMap<String, Vector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vectors {
    pub vectors_options: Option<vectors::VectorsOptions>,
}
/// Nested message and enum types in `Vectors`.
pub mod vectors {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum VectorsOptions {
        Vector(super::Vector),
        Vectors(super::NamedVectors),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorsSelector {
    /// List of vectors to include into result
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithVectorsSelector {
    pub selector_options: Option<with_vectors_selector::SelectorOptions>,
}
/// Nested message and enum types in `WithVectorsSelector`.
pub mod with_vectors_selector {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SelectorOptions {
        /// If `true` - return all vectors, if `false` - none
        Enable(bool),
        /// List of payload keys to include into result
        Include(super::VectorsSelector),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationSearchParams {
    ///
    /// If set to true, search will ignore quantized vector data
    pub ignore: Option<bool>,
    ///
    /// If true, use original vectors to re-score top-k results. If ignored, qdrant decides automatically does rescore enabled or not.
    pub rescore: Option<bool>,
    ///
    /// Oversampling factor for quantization.
    ///
    /// Defines how many extra vectors should be pre-selected using quantized index,
    /// and then re-scored using original vectors.
    ///
    /// For example, if `oversampling` is 2.4 and `limit` is 100, then 240 vectors will be pre-selected using quantized index,
    /// and then top-100 will be returned after re-scoring.
    pub oversampling: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    ///
    /// Params relevant to HNSW index. Size of the beam in a beam-search.
    /// Larger the value - more accurate the result, more time required for search.
    pub hnsw_ef: Option<u64>,
    ///
    /// Search without approximation. If set to true, search may run long but with exact results.
    pub exact: Option<bool>,
    ///
    /// If set to true, search will ignore quantized vector data
    pub quantization: Option<QuantizationSearchParams>,
    ///
    /// If enabled, the engine will only perform search among indexed or small segments.
    /// Using this option prevents slow searches in case of delayed index, but does not
    /// guarantee that all uploaded vectors will be included in search results
    pub indexed_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithLookup {
    /// Name of the collection to use for points lookup
    pub collection: String,
    /// Options for specifying which payload to include (or not)
    pub with_payload: Option<WithPayloadSelector>,
    /// Options for specifying which vectors to include (or not)
    pub with_vectors: Option<WithVectorsSelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsOperationResponse {
    pub result: Option<UpdateResult>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for PointsOperationResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    /// Number of operation
    pub operation_id: Option<u64>,
    /// Operation status
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredPoint {
    /// Point id
    pub id: Option<PointId>,
    /// Payload
    pub payload: ::std::collections::HashMap<String, Value>,
    /// Similarity score
    pub score: f32,
    /// Last update operation applied to this point
    pub version: u64,
    /// Vectors to search
    pub vectors: Option<Vectors>,
    /// Shard key
    pub shard_key: Option<ShardKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupId {
    pub kind: Option<group_id::Kind>,
}
/// Nested message and enum types in `GroupId`.
pub mod group_id {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Kind {
        /// Represents a double value.
        UnsignedValue(u64),
        /// Represents an integer value
        IntegerValue(i64),
        /// Represents a string value.
        StringValue(String),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointGroup {
    /// Group id
    pub id: Option<GroupId>,
    /// Points in the group
    pub hits: Vec<ScoredPoint>,
    /// Point(s) from the lookup collection that matches the group id
    pub lookup: Option<RetrievedPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupsResult {
    /// Groups
    pub groups: Vec<PointGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub result: Vec<ScoredPoint>,
    /// Time spent to process
    pub time: f64,
}

impl IntoResponse for SearchResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub result: Vec<ScoredPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchBatchResponse {
    pub result: Vec<BatchResult>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for SearchBatchResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchGroupsResponse {
    pub result: Option<GroupsResult>,
    /// Time spent to process
    pub time: f64,
}

impl IntoResponse for SearchGroupsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountResponse {
    pub result: Option<CountResult>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for CountResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollResponse {
    /// Use this offset for the next query
    pub next_page_offset: Option<PointId>,

    pub result: Vec<RetrievedPoint>,
    /// Time spent to process
    pub time: f64,
}

impl IntoResponse for ScrollResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountResult {
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedPoint {
    pub id: Option<PointId>,

    pub payload: ::std::collections::HashMap<String, Value>,

    pub vectors: Option<Vectors>,
    /// Shard key
    pub shard_key: Option<ShardKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResponse {
    pub result: Vec<RetrievedPoint>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for GetResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendResponse {
    pub result: Vec<ScoredPoint>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for RecommendResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendBatchResponse {
    pub result: Vec<BatchResult>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for RecommendBatchResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverResponse {
    pub result: Vec<ScoredPoint>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for DiscoverResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverBatchResponse {
    pub result: Vec<BatchResult>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for DiscoverBatchResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendGroupsResponse {
    pub result: Option<GroupsResult>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for RecommendGroupsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBatchResponse {
    pub result: Vec<UpdateResult>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for UpdateBatchResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsEmptyCondition {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsNullCondition {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasIdCondition {
    pub has_id: Vec<PointId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub match_value: Option<r#match::MatchValue>,
}
/// Nested message and enum types in `Match`.
pub mod r#match {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum MatchValue {
        /// Match string keyword
        Keyword(String),
        /// Match integer
        Integer(i64),
        /// Match boolean
        Boolean(bool),
        /// Match text
        Text(String),
        /// Match multiple keywords
        Keywords(super::RepeatedStrings),
        /// Match multiple integers
        Integers(super::RepeatedIntegers),
        /// Match any other value except those integers
        ExceptIntegers(super::RepeatedIntegers),
        /// Match any other value except those keywords
        ExceptKeywords(super::RepeatedStrings),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatedStrings {
    pub strings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatedIntegers {
    pub integers: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub lt: Option<f64>,

    pub gt: Option<f64>,

    pub gte: Option<f64>,

    pub lte: Option<f64>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DatetimeRange {

//     pub lt: Option<Timestamp>,

//     pub gt: Option<Timestamp>,

//     pub gte: Option<Timestamp>,

//     pub lte: Option<Timestamp>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoBoundingBox {
    /// north-west corner
    pub top_left: Option<GeoPoint>,
    /// south-east corner
    pub bottom_right: Option<GeoPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoRadius {
    /// Center of the circle
    pub center: Option<GeoPoint>,
    /// In meters
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLineString {
    /// Ordered sequence of GeoPoints representing the line
    pub points: Vec<GeoPoint>,
}
/// For a valid GeoPolygon, both the exterior and interior GeoLineStrings must consist of a minimum of 4 points.
/// Additionally, the first and last points of each GeoLineString must be the same.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPolygon {
    /// The exterior line bounds the surface
    pub exterior: Option<GeoLineString>,
    /// Interior lines (if present) bound holes within the surface
    pub interiors: Vec<GeoLineString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesCount {
    pub lt: Option<u64>,

    pub gt: Option<u64>,

    pub gte: Option<u64>,

    pub lte: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsIdsList {
    pub ids: Vec<PointId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointStruct {
    pub id: Option<PointId>,
    pub payload: ::std::collections::HashMap<String, Value>,
    pub vectors: Option<Vectors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lon: f64,

    pub lat: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum WriteOrderingType {
    /// Write operations may be reordered, works faster, default
    Weak = 0,
    /// Write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change
    Medium = 1,
    /// Write operations go through the permanent leader, consistent, but may be unavailable if leader is down
    Strong = 2,
}
impl WriteOrderingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WriteOrderingType::Weak => "Weak",
            WriteOrderingType::Medium => "Medium",
            WriteOrderingType::Strong => "Strong",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "Weak" => Some(Self::Weak),
            "Medium" => Some(Self::Medium),
            "Strong" => Some(Self::Strong),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ReadConsistencyType {
    /// Send request to all nodes and return points which are present on all of them
    All = 0,
    /// Send requests to all nodes and return points which are present on majority of them
    Majority = 1,
    /// Send requests to half + 1 nodes, return points which are present on all of them
    Quorum = 2,
}
impl ReadConsistencyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReadConsistencyType::All => "All",
            ReadConsistencyType::Majority => "Majority",
            ReadConsistencyType::Quorum => "Quorum",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "All" => Some(Self::All),
            "Majority" => Some(Self::Majority),
            "Quorum" => Some(Self::Quorum),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum FieldType {
    Keyword = 0,
    Integer = 1,
    Float = 2,
    Geo = 3,
    Text = 4,
    Bool = 5,
    Datetime = 6,
}
impl FieldType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FieldType::Keyword => "FieldTypeKeyword",
            FieldType::Integer => "FieldTypeInteger",
            FieldType::Float => "FieldTypeFloat",
            FieldType::Geo => "FieldTypeGeo",
            FieldType::Text => "FieldTypeText",
            FieldType::Bool => "FieldTypeBool",
            FieldType::Datetime => "FieldTypeDatetime",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "FieldTypeKeyword" => Some(Self::Keyword),
            "FieldTypeInteger" => Some(Self::Integer),
            "FieldTypeFloat" => Some(Self::Float),
            "FieldTypeGeo" => Some(Self::Geo),
            "FieldTypeText" => Some(Self::Text),
            "FieldTypeBool" => Some(Self::Bool),
            "FieldTypeDatetime" => Some(Self::Datetime),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Direction {
    Asc = 0,
    Desc = 1,
}
impl Direction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Direction::Asc => "Asc",
            Direction::Desc => "Desc",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "Asc" => Some(Self::Asc),
            "Desc" => Some(Self::Desc),
            _ => None,
        }
    }
}
/// How to use positive and negative vectors to find the results, default is `AverageVector`:
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum RecommendStrategy {
    /// Average positive and negative vectors and create a single query with the formula
    /// `query = avg_pos + avg_pos - avg_neg`. Then performs normal search.
    AverageVector = 0,
    /// Uses custom search objective. Each candidate is compared against all
    /// examples, its score is then chosen from the `max(max_pos_score, max_neg_score)`.
    /// If the `max_neg_score` is chosen then it is squared and negated.
    BestScore = 1,
}
impl RecommendStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RecommendStrategy::AverageVector => "AverageVector",
            RecommendStrategy::BestScore => "BestScore",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "AverageVector" => Some(Self::AverageVector),
            "BestScore" => Some(Self::BestScore),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum UpdateStatus {
    UnknownUpdateStatus = 0,
    /// Update is received, but not processed yet
    Acknowledged = 1,
    /// Update is applied and ready for search
    Completed = 2,
    /// Internal: update is rejected due to an outdated clock
    ClockRejected = 3,
}
impl UpdateStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateStatus::UnknownUpdateStatus => "UnknownUpdateStatus",
            UpdateStatus::Acknowledged => "Acknowledged",
            UpdateStatus::Completed => "Completed",
            UpdateStatus::ClockRejected => "ClockRejected",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "UnknownUpdateStatus" => Some(Self::UnknownUpdateStatus),
            "Acknowledged" => Some(Self::Acknowledged),
            "Completed" => Some(Self::Completed),
            "ClockRejected" => Some(Self::ClockRejected),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFullSnapshotRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFullSnapshotsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFullSnapshotRequest {
    /// Name of the full snapshot
    pub snapshot_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    /// Name of the collection
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSnapshotsRequest {
    /// Name of the collection
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSnapshotRequest {
    /// Name of the collection
    pub collection_name: String,
    /// Name of the collection snapshot
    pub snapshot_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDescription {
    /// Name of the snapshot
    pub name: String,
    /// Creation time of the snapshot
    // pub creation_time: Option<::prost_types::Timestamp>,
    /// Size of the snapshot in bytes
    pub size: i64,
    /// SHA256 digest of the snapshot file
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotResponse {
    pub snapshot_description: Option<SnapshotDescription>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for CreateSnapshotResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSnapshotsResponse {
    pub snapshot_descriptions: Vec<SnapshotDescription>,
    /// Time spent to process
    pub time: f64,
}

impl IntoResponse for ListSnapshotsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSnapshotResponse {
    /// Time spent to process
    pub time: f64,
}

impl IntoResponse for DeleteSnapshotResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckReply {
    pub title: String,
    pub version: String,
    pub commit: Option<String>,
}
