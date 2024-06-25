use crate::pb::engine::qdrant::collection;
use anyhow;
use http::Method;
use http::StatusCode;
use lazy_static::lazy_static;
use reqwest::{Client, ClientBuilder, Request, Url};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_value};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, instrument, warn};

/*
    Get(ctx context.Context, in *GetCollectionInfoRequest, opts ...grpc.CallOption) (*GetCollectionInfoResponse, error)
    // Get list name of all existing collections
    List(ctx context.Context, in *ListCollectionsRequest, opts ...grpc.CallOption) (*ListCollectionsResponse, error)
    // Create new collection with given parameters
    Create(ctx context.Context, in *CreateCollection, opts ...grpc.CallOption) (*CollectionOperationResponse, error)
    // Update parameters of the existing collection
    Update(ctx context.Context, in *UpdateCollection, opts ...grpc.CallOption) (*CollectionOperationResponse, error)
    // Drop collection and all associated data
    Delete(ctx context.Context, in *DeleteCollection, opts ...grpc.CallOption) (*CollectionOperationResponse, error)
    // Update Aliases of the existing collection
    UpdateAliases(ctx context.Context, in *ChangeAliases, opts ...grpc.CallOption) (*CollectionOperationResponse, error)
    // Get list of all aliases for a collection
    ListCollectionAliases(ctx context.Context, in *ListCollectionAliasesRequest, opts ...grpc.CallOption) (*ListAliasesResponse, error)
    // Get list of all aliases for all existing collections
    ListAliases(ctx context.Context, in *ListAliasesRequest, opts ...grpc.CallOption) (*ListAliasesResponse, error)
    // Get cluster information for a collection
    CollectionClusterInfo(ctx context.Context, in *CollectionClusterInfoRequest, opts ...grpc.CallOption) (*CollectionClusterInfoResponse, error)
    // Check the existence of a collection
    CollectionExists(ctx context.Context, in *CollectionExistsRequest, opts ...grpc.CallOption) (*CollectionExistsResponse, error)
    // Update cluster setup for a collection
    UpdateCollectionClusterSetup(ctx context.Context, in *UpdateCollectionClusterSetupRequest, opts ...grpc.CallOption) (*UpdateCollectionClusterSetupResponse, error)
    // Create shard key
    CreateShardKey(ctx context.Context, in *CreateShardKeyRequest, opts ...grpc.CallOption) (*CreateShardKeyResponse, error)
    // Delete shard key
    DeleteShardKey(ctx context.Context, in *DeleteShardKeyRequest, opts ...grpc.CallOption) (*DeleteShardKeyResponse, error)
*/

pub async fn get(
    host: impl AsRef<str>,
    collection_name: impl AsRef<str>,
) -> anyhow::Result<collection::GetCollectionInfoResponse> {
    let client = Client::new();
    let url = format!(
        "http://{host}/collections/{name}",
        host = host.as_ref(),
        name = collection_name.as_ref()
    );
    let response = client.get(url).send().await?;
    let status_code = response.status().as_u16();
    info!("create response status={:?}", status_code);
    if status_code < 400 {
        let body = response.text().await?;
        let resp = serde_json::from_str::<collection::GetCollectionInfoResponse>(&body)?;
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            response.status().as_u16()
        ))
    }
}

pub async fn create(
    host: impl AsRef<str>,
    req: collection::CreateCollection,
    // body: serde_json::Value, // 这里是通过配置生成的json value
) -> anyhow::Result<collection::CollectionOperationResponse> {
    info!("qdrant create collection");
    let client = Client::new();
    let url = format!(
        "http://{host}/collections/{name}",
        host = host.as_ref(),
        name = &(req.collection_name)
    );
    info!("url={:?}", url);
    // let data = json!({   //直接序列化后的value
    //     "name": "Alice",
    //     "age": 20
    // });
    // 通过构造好的value
    let response = client
        .put(url)
        // .header("Content-Type", "application/json")
        .json(&req) //enable features json
        .send()
        .await?;
    // serde_json::to_value(value)

    let code = response.status().as_u16();
    info!("create response status={:?}", code);
    if code < 400 {
        let body = response.text().await?;
        let resp = serde_json::from_str::<collection::CollectionOperationResponse>(&body)?;
        println!("response received {:?}", resp);
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            response.status().as_u16()
        ))
    }
}

pub async fn list(host: impl AsRef<str>) -> anyhow::Result<collection::ListCollectionsResponse> {
    let url = format!("http://{host}/collections", host = host.as_ref());
    let client = Client::new();

    let response = client.get(&url).send().await?;
    let code = response.status().as_u16();

    if code < 400 {
        let body = response.text().await?; // moved here
        let resp = serde_json::from_str::<collection::ListCollectionsResponse>(&body)?;
        println!("response received {:?}", resp);
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            response.status().as_u16()
        ))
    }
}

/*
PATCH /collections/{collection_name}
{
    "optimizers_config": {
        "indexing_threshold": 10000
    }
}
*/
pub async fn update(
    host: impl AsRef<str>,
    collection_name: impl AsRef<str>,
    req: collection::UpdateCollection,
) -> anyhow::Result<collection::CollectionOperationResponse> {
    let url = format!(
        "http://{host}/collections/{name}",
        host = host.as_ref(),
        name = collection_name.as_ref()
    );
    let client = Client::new();

    let response = client.patch(&url).json(&req).send().await?;
    let code = response.status().as_u16();

    if code < 400 {
        let body = response.text().await?; // moved here
        let resp = serde_json::from_str::<collection::CollectionOperationResponse>(&body)?;
        println!("response received {:?}", resp);
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            response.status().as_u16()
        ))
    }
}

/*
POST /collections/aliases
{
    "actions": [
        {
            "create_alias": {
                "collection_name": "example_collection",
                "alias_name": "production_collection"
            }
        }
    ]
}

POST /collections/aliases
{
    "actions": [
        {
            "delete_alias": {
                "alias_name": "production_collection"
            }
        }
    ]
}
switch alias

POST /collections/aliases
{
    "actions": [
        {
            "delete_alias": {
                "alias_name": "production_collection"
            }
        },
        {
            "create_alias": {
                "collection_name": "example_collection",
                "alias_name": "production_collection"
            }
        }
    ]
}
*/
/// 支持增删alias
pub async fn update_alias(
    host: impl AsRef<str>,
    req: collection::ChangeAliases,
    // body: serde_json::Value,
) -> anyhow::Result<collection::CollectionOperationResponse> {
    let url = format!("http://{host}/collections/aliases", host = host.as_ref());
    let client = Client::new();
    let response = client.post(&url).json(&req).send().await?;
    let code = response.status().as_u16();
    if code < 400 {
        let body = response.text().await?;
        let resp = serde_json::from_str(&body)?;
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            response.status().as_u16()
        ))
    }
}

/*
GET /collections/{collection_name}/aliases
*/
pub async fn list_alias(
    host: impl AsRef<str>,
    collection_name: impl AsRef<str>,
) -> anyhow::Result<collection::ListAliasesResponse> {
    let url = format!(
        "http://{host}/collections/{name}/aliases",
        host = host.as_ref(),
        name = collection_name.as_ref()
    );
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let code = response.status().as_u16();
    if code < 400 {
        let body = response.text().await?;
        let resp = serde_json::from_str(&body)?;
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            response.status().as_u16()
        ))
    }
}

// DELETE http://localhost:6333/collections/{collection_name}
pub async fn delete(
    host: impl AsRef<str>,
    collection_name: impl AsRef<str>,
) -> anyhow::Result<collection::CollectionOperationResponse> {
    let url = format!(
        "http://{host}/collections/{name}",
        host = host.as_ref(),
        name = collection_name.as_ref()
    );
    let client = Client::new();
    let response = client.delete(url).send().await?;
    let code = response.status().as_u16();
    if code < 400 {
        let body = response.text().await?;
        let resp = serde_json::from_str(&body)?;
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            response.status().as_u16()
        ))
    }
}

// pub async fn payload_index(host: impl AsRef<str>, name: impl AsRef<str>, body: serde_json::Value) {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_collections() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("Hello from tokio!");
            rt.spawn(async {
                list("localhost:6333").await;
            })
            .await
            .unwrap();
        });
    }
}
