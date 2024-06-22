use crate::pb::engine::qdrant::collection;
use anyhow;
use anyhow::Ok;
use http::Method;
use http::StatusCode;
use lazy_static::lazy_static;
use reqwest::{Client, ClientBuilder, Request, Url};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

pub async fn get_collection(
    host: impl AsRef<str>,
    name: impl AsRef<str>,
) -> anyhow::Result<collection::GetCollectionInfoResponse> {
    let client = Client::new();
    let url = format!(
        "http://{host}/collections/{name}",
        host = host.as_ref(),
        name = name.as_ref()
    );
    let response = client.get(url).send().await?;
    let status_code = response.status().as_u16();
    if status_code == 200 {
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

pub async fn create_collection(
    host: impl AsRef<str>,
    name: impl AsRef<str>,
    body: serde_json::Value, // 这里是通过配置生成的jsonvalue
) -> anyhow::Result<collection::CollectionOperationResponse> {
    let client = Client::new();
    let url = format!("http://{host}/collections", host = host.as_ref());
    // let data = json!({   //直接序列化后的value
    //     "name": "Alice",
    //     "age": 20
    // });
    // 通过构造好的value
    let response = client
        .post(url)
        // .header("Content-Type", "application/json")
        .json(&body) //enable features json
        .send()
        .await?;
    // serde_json::to_value(value)

    let code = response.status().as_u16();
    if code == 200 {
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

pub async fn list_collections(
    host: impl AsRef<str>,
) -> anyhow::Result<collection::ListCollectionsResponse> {
    let url = format!("http://{host}/collections", host = host.as_ref());
    let client = Client::new();

    let response = client.get(&url).send().await?;
    let code = response.status().as_u16();

    if code == 200 {
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
pub async fn update_collection(
    host: impl AsRef<str>,
    name: impl AsRef<str>,
    body: serde_json::Value,
) {
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
    body: serde_json::Value,
) -> anyhow::Result<collection::CollectionOperationResponse> {
    let url = format!("http://{host}/collections/aliases", host = host.as_ref());
    let client = Client::new();
    let response = client.post(&url).send().await?;
    let code = response.status().as_u16();
    if code == 200 {
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
pub async fn list_collection_alias(
    host: impl AsRef<str>,
    name: impl AsRef<str>,
) -> anyhow::Result<collection::ListAliasesResponse> {
    let url = format!(
        "http://{host}/collections/{name}/aliases",
        host = host.as_ref(),
        name = name.as_ref()
    );
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let code = response.status().as_u16();
    if code == 200 {
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
pub async fn delete_collection(
    host: impl AsRef<str>,
    name: impl AsRef<str>,
) -> anyhow::Result<collection::CollectionOperationResponse> {
    let url = format!(
        "http://{host}/collections/{name}",
        host = host.as_ref(),
        name = name.as_ref()
    );
    let client = Client::new();
    let response = client.delete(url).send().await?;
    let code = response.status().as_u16();
    if code == 200 {
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

pub async fn payload_index(host: impl AsRef<str>, name: impl AsRef<str>, body: serde_json::Value) {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_collections() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("Hello from tokio!");
            rt.spawn(async {
                list_collections("localhost:6333").await;
            })
            .await
            .unwrap();
        });
    }
}
