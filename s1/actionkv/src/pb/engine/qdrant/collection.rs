use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionDescription {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseResult {
    pub collections: Vec<CollectionDescription>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCollectionsResponse {
    pub result: ResponseResult,
    pub status: String,
    pub time: f64,
}
impl IntoResponse for ListCollectionsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorParams {
    /// Size of the vectors
    pub size: u64,
    /// Distance function used for comparing vectors
    pub distance: String,
    /// Configuration of vector HNSW graph. If omitted - the collection configuration will be used
    pub hnsw_config: Option<HnswConfigDiff>,
    /// Configuration of vector quantization config. If omitted - the collection configuration will be used
    pub quantization_config: Option<QuantizationConfig>,
    /// If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM.
    pub on_disk: Option<bool>,
    /// Data type of the vectors
    pub datatype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorParamsDiff {
    /// Update params for HNSW index. If empty object - it will be unset
    pub hnsw_config: Option<HnswConfigDiff>,
    /// Update quantization params. If none - it is left unchanged.
    pub quantization_config: Option<QuantizationConfigDiff>,
    /// If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM.
    pub on_disk: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorParamsMap {
    pub map: ::std::collections::HashMap<String, VectorParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorParamsDiffMap {
    pub map: ::std::collections::HashMap<String, VectorParamsDiff>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorsConfig {
    pub config: Option<vectors_config::Config>,
}
/// Nested message and enum types in `VectorsConfig`.
pub mod vectors_config {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Config {
        Params(super::VectorParams),
        ParamsMap(super::VectorParamsMap),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorsConfigDiff {
    pub config: Option<vectors_config_diff::Config>,
}
/// Nested message and enum types in `VectorsConfigDiff`.
pub mod vectors_config_diff {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Config {
        Params(super::VectorParamsDiff),

        ParamsMap(super::VectorParamsDiffMap),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseVectorParams {
    /// Configuration of sparse index
    pub index: Option<SparseIndexConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseVectorConfig {
    pub map: ::std::collections::HashMap<String, SparseVectorParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCollectionInfoRequest {
    /// Name of the collection
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionExistsRequest {
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionExists {
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionExistsResponse {
    pub result: Option<CollectionExists>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for CollectionExistsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCollectionsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCollectionInfoResponse {
    pub result: Option<CollectionInfo>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for GetCollectionInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerStatus {
    pub ok: bool,

    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HnswConfigDiff {
    ///
    /// Number of edges per node in the index graph. Larger the value - more accurate the search, more space required.
    pub m: Option<u64>,
    ///
    /// Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build the index.
    pub ef_construct: Option<u64>,
    ///
    /// Minimal size (in KiloBytes) of vectors for additional payload-based indexing.
    /// If the payload chunk is smaller than `full_scan_threshold` additional indexing won't be used -
    /// in this case full-scan search should be preferred by query planner and additional indexing is not required.
    /// Note: 1 Kb = 1 vector of size 256
    pub full_scan_threshold: Option<u64>,
    ///
    /// Number of parallel threads used for background index building.
    /// If 0 - automatically select from 8 to 16.
    /// Best to keep between 8 and 16 to prevent likelihood of building broken/inefficient HNSW graphs.
    /// On small CPUs, less threads are used.
    pub max_indexing_threads: Option<u64>,
    ///
    /// Store HNSW index on disk. If set to false, the index will be stored in RAM.
    pub on_disk: Option<bool>,
    ///
    /// Number of additional payload-aware links per node in the index graph. If not set - regular M parameter will be used.
    pub payload_m: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseIndexConfig {
    ///
    /// Prefer a full scan search upto (excluding) this number of vectors.
    /// Note: this is number of vectors, not KiloBytes.
    pub full_scan_threshold: Option<u64>,
    ///
    /// Store inverted index on disk. If set to false, the index will be stored in RAM.
    pub on_disk: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalConfigDiff {
    /// Size of a single WAL block file
    pub wal_capacity_mb: Option<u64>,
    /// Number of segments to create in advance
    pub wal_segments_ahead: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptimizersConfigDiff {
    ///
    /// The minimal fraction of deleted vectors in a segment, required to perform segment optimization
    pub deleted_threshold: Option<f64>,
    ///
    /// The minimal number of vectors in a segment, required to perform segment optimization
    pub vacuum_min_vector_number: Option<u64>,
    ///
    /// Target amount of segments the optimizer will try to keep.
    /// Real amount of segments may vary depending on multiple parameters:
    ///
    /// - Amount of stored points.
    /// - Current write RPS.
    ///
    /// It is recommended to select the default number of segments as a factor of the number of search threads,
    /// so that each segment would be handled evenly by one of the threads.
    pub default_segment_number: Option<u64>,
    ///
    /// Do not create segments larger this size (in kilobytes).
    /// Large segments might require disproportionately long indexation times,
    /// therefore it makes sense to limit the size of segments.
    ///
    /// If indexing speed is more important - make this parameter lower.
    /// If search speed is more important - make this parameter higher.
    /// Note: 1Kb = 1 vector of size 256
    /// If not set, will be automatically selected considering the number of available CPUs.
    pub max_segment_size: Option<u64>,
    ///
    /// Maximum size (in kilobytes) of vectors to store in-memory per segment.
    /// Segments larger than this threshold will be stored as read-only memmaped file.
    ///
    /// Memmap storage is disabled by default, to enable it, set this threshold to a reasonable value.
    ///
    /// To disable memmap storage, set this to `0`.
    ///
    /// Note: 1Kb = 1 vector of size 256
    pub memmap_threshold: Option<u64>,
    ///
    /// Maximum size (in kilobytes) of vectors allowed for plain index, exceeding this threshold will enable vector indexing
    ///
    /// Default value is 20,000, based on <<https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md>.>
    ///
    /// To disable vector indexing, set to `0`.
    ///
    /// Note: 1kB = 1 vector of size 256.
    pub indexing_threshold: Option<u64>,
    ///
    /// Interval between forced flushes.
    pub flush_interval_sec: Option<u64>,
    ///
    /// Max number of threads (jobs) for running optimizations per shard.
    /// Note: each optimization job will also use `max_indexing_threads` threads by itself for index building.
    /// If null - have no limit and choose dynamically to saturate CPU.
    /// If 0 - no optimization threads, optimizations will be disabled.
    pub max_optimization_threads: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalarQuantization {
    /// Type of quantization
    pub r#type: i32,
    /// Number of bits to use for quantization
    pub quantile: Option<f32>,
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub always_ram: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductQuantization {
    /// Compression ratio
    pub compression: i32,
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub always_ram: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryQuantization {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub always_ram: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationConfig {
    pub quantization: Option<quantization_config::Quantization>,
}
/// Nested message and enum types in `QuantizationConfig`.
pub mod quantization_config {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Quantization {
        Scalar(super::ScalarQuantization),

        Product(super::ProductQuantization),

        Binary(super::BinaryQuantization),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disabled {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationConfigDiff {
    pub quantization: Option<quantization_config_diff::Quantization>,
}
/// Nested message and enum types in `QuantizationConfigDiff`.
pub mod quantization_config_diff {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Quantization {
        Scalar(super::ScalarQuantization),

        Product(super::ProductQuantization),

        Disabled(super::Disabled),

        Binary(super::BinaryQuantization),
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCollection {
    /// Name of the collection
    pub collection_name: String,
    /// Configuration of vector index
    pub hnsw_config: Option<HnswConfigDiff>,
    /// Configuration of the Write-Ahead-Log
    pub wal_config: Option<WalConfigDiff>,
    /// Configuration of the optimizers
    pub optimizers_config: Option<OptimizersConfigDiff>,
    /// Number of shards in the collection, default is 1 for standalone, otherwise equal to the number of nodes. Minimum is 1
    pub shard_number: Option<u32>,
    /// If true - point's payload will not be stored in memory
    pub on_disk_payload: Option<bool>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub timeout: Option<u64>,
    /// Configuration for vectors
    pub vectors: Option<VectorParams>, //teddy 修改成json需要的格式
    /// Number of replicas of each shard that network tries to maintain, default = 1
    pub replication_factor: Option<u32>,
    /// How many replicas should apply the operation for us to consider it successful, default = 1
    pub write_consistency_factor: Option<u32>,
    /// Specify name of the other collection to copy data from
    pub init_from_collection: Option<String>,
    /// Quantization configuration of vector
    pub quantization_config: Option<QuantizationConfig>,
    /// Sharding method
    pub sharding_method: Option<i32>,
    /// Configuration for sparse vectors
    pub sparse_vectors_config: Option<SparseVectorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCollection {
    /// Name of the collection
    pub collection_name: String,
    /// New configuration parameters for the collection. This operation is blocking, it will only proceed once all current optimizations are complete
    pub optimizers_config: Option<OptimizersConfigDiff>,
    /// Wait timeout for operation commit in seconds if blocking, if not specified - default value will be supplied
    pub timeout: Option<u64>,
    /// New configuration parameters for the collection
    pub params: Option<CollectionParamsDiff>,
    /// New HNSW parameters for the collection index
    pub hnsw_config: Option<HnswConfigDiff>,
    /// New vector parameters
    pub vectors_config: Option<VectorsConfigDiff>,
    /// Quantization configuration of vector
    pub quantization_config: Option<QuantizationConfigDiff>,
    /// New sparse vector parameters
    pub sparse_vectors_config: Option<SparseVectorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCollection {
    /// Name of the collection
    pub collection_name: String,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionOperationResponse {
    /// if operation made changes
    pub result: bool,
    /// Time spent to process
    pub time: f64,
}

impl IntoResponse for CollectionOperationResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionParams {
    /// Number of shards in collection
    pub shard_number: u32,
    /// If true - point's payload will not be stored in memory
    pub on_disk_payload: bool,
    /// Configuration for vectors
    pub vectors_config: Option<VectorsConfig>,
    /// Number of replicas of each shard that network tries to maintain
    pub replication_factor: Option<u32>,
    /// How many replicas should apply the operation for us to consider it successful
    pub write_consistency_factor: Option<u32>,
    /// Fan-out every read request to these many additional remote nodes (and return first available response)
    pub read_fan_out_factor: Option<u32>,
    /// Sharding method
    pub sharding_method: Option<i32>,
    /// Configuration for sparse vectors
    pub sparse_vectors_config: Option<SparseVectorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionParamsDiff {
    /// Number of replicas of each shard that network tries to maintain
    pub replication_factor: Option<u32>,
    /// How many replicas should apply the operation for us to consider it successful
    pub write_consistency_factor: Option<u32>,
    /// If true - point's payload will not be stored in memory
    pub on_disk_payload: Option<bool>,
    /// Fan-out every read request to these many additional remote nodes (and return first available response)
    pub read_fan_out_factor: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    /// Collection parameters
    pub params: Option<CollectionParams>,
    /// Configuration of vector index
    pub hnsw_config: Option<HnswConfigDiff>,
    /// Configuration of the optimizers
    pub optimizer_config: Option<OptimizersConfigDiff>,
    /// Configuration of the Write-Ahead-Log
    pub wal_config: Option<WalConfigDiff>,
    /// Configuration of the vector quantization
    pub quantization_config: Option<QuantizationConfig>,
}

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
    pub index_params: Option<payload_index_params::IndexParams>,
}
/// Nested message and enum types in `PayloadIndexParams`.
pub mod payload_index_params {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    /// operating condition of the collection
    pub status: String,
    /// status of collection optimizers
    pub optimizer_status: Option<OptimizerStatus>,
    /// Approximate number of vectors in the collection
    pub vectors_count: Option<u64>,
    /// Number of independent segments
    pub segments_count: u64,
    /// Configuration
    pub config: Option<CollectionConfig>,
    /// Collection data types
    pub payload_schema: ::std::collections::HashMap<String, PayloadSchemaInfo>,
    /// Approximate number of points in the collection
    pub points_count: Option<u64>,
    /// Approximate number of indexed vectors in the collection.
    pub indexed_vectors_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAliases {
    /// List of actions
    pub actions: Vec<AliasOperations>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasOperations {
    #[serde(flatten)]
    pub action: Option<alias_operations::Action>,
}
/// Nested message and enum types in `AliasOperations`.
pub mod alias_operations {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Action {
        #[serde(rename="create_alias")]
        CreateAlias(super::CreateAlias),
        #[serde(rename="rename_alias")]
        RenameAlias(super::RenameAlias),
        #[serde(rename="delete_alias")]
        DeleteAlias(super::DeleteAlias),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlias {
    /// Name of the collection
    pub collection_name: String,
    /// New name of the alias
    pub alias_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameAlias {
    /// Name of the alias to rename
    pub old_alias_name: String,
    /// Name of the alias
    pub new_alias_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAlias {
    /// Name of the alias
    pub alias_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAliasesRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCollectionAliasesRequest {
    /// Name of the collection
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasDescription {
    /// Name of the alias
    pub alias_name: String,
    /// Name of the collection
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAliasesResponse {
    pub aliases: Vec<AliasDescription>,
    /// Time spent to process
    pub time: f64,
}
impl IntoResponse for ListAliasesResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionClusterInfoRequest {
    /// Name of the collection
    pub collection_name: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionClusterInfoResponse {
    /// ID of this peer
    pub peer_id: u64,
    /// Total number of shards
    pub shard_count: u64,
    /// Local shards
    pub local_shards: Vec<LocalShardInfo>,
    /// Remote shards
    pub remote_shards: Vec<RemoteShardInfo>,
    /// Shard transfers
    pub shard_transfers: Vec<ShardTransferInfo>,
}

impl IntoResponse for CollectionClusterInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCollectionClusterSetupRequest {
    /// Name of the collection
    pub collection_name: String,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub timeout: Option<u64>,
    pub operation: Option<update_collection_cluster_setup_request::Operation>,
}
/// Nested message and enum types in `UpdateCollectionClusterSetupRequest`.
pub mod update_collection_cluster_setup_request {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Operation {
        MoveShard(super::MoveShard),

        ReplicateShard(super::MoveShard),

        AbortTransfer(super::AbortShardTransfer),

        DropReplica(super::Replica),

        CreateShardKey(super::CreateShardKey),

        DeleteShardKey(super::DeleteShardKey),

        RestartTransfer(super::RestartTransfer),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCollectionClusterSetupResponse {
    pub result: bool,
}

impl IntoResponse for UpdateCollectionClusterSetupResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShardKeyRequest {
    /// Name of the collection
    pub collection_name: String,
    /// Request to create shard key
    pub request: Option<CreateShardKey>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteShardKeyRequest {
    /// Name of the collection
    pub collection_name: String,
    /// Request to delete shard key
    pub request: Option<DeleteShardKey>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShardKeyResponse {
    pub result: bool,
}

impl IntoResponse for CreateShardKeyResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteShardKeyResponse {
    pub result: bool,
}

impl IntoResponse for DeleteShardKeyResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(i32)]
pub enum Distance {
    UnknownDistance = 0,
    Cosine = 1,
    Euclid = 2,
    Dot = 3,
    Manhattan = 4,
}
impl Distance {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Distance::UnknownDistance => "UnknownDistance",
            Distance::Cosine => "Cosine",
            Distance::Euclid => "Euclid",
            Distance::Dot => "Dot",
            Distance::Manhattan => "Manhattan",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "UnknownDistance" => Some(Self::UnknownDistance),
            "Cosine" => Some(Self::Cosine),
            "Euclid" => Some(Self::Euclid),
            "Dot" => Some(Self::Dot),
            "Manhattan" => Some(Self::Manhattan),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(i32)]
pub enum CollectionStatus {
    UnknownCollectionStatus = 0,
    /// All segments are ready
    Green = 1,
    /// Optimization in process
    Yellow = 2,
    /// Something went wrong
    Red = 3,
    /// Optimization is pending
    Grey = 4,
}
impl CollectionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CollectionStatus::UnknownCollectionStatus => "UnknownCollectionStatus",
            CollectionStatus::Green => "Green",
            CollectionStatus::Yellow => "Yellow",
            CollectionStatus::Red => "Red",
            CollectionStatus::Grey => "Grey",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "UnknownCollectionStatus" => Some(Self::UnknownCollectionStatus),
            "Green" => Some(Self::Green),
            "Yellow" => Some(Self::Yellow),
            "Red" => Some(Self::Red),
            "Grey" => Some(Self::Grey),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
