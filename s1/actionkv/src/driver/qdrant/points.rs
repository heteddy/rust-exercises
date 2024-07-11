use crate::pb::engine::qdrant::points::{
    point_id, vectors, CountResponse, DeletePoints, DiscoverBatchResponse, DiscoverResponse,
    GetPoints, GetResponse, PointId, PointStruct, PointsOperationResponse, RecommendBatchResponse,
    RecommendGroupsResponse, RecommendResponse, RetrievedPoint, ScoredPoint, ScrollResponse,
    SearchBatchResponse, SearchResponse, UpdateBatchResponse, UpsertPoints,
};
use anyhow;
use handlebars::Handlebars;
use http::Method;
use http::StatusCode;
use lazy_static::lazy_static;
use reqwest::{Client, ClientBuilder, Request, Url};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Map, Number, Value};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, instrument, warn};
/*
Upsert(context.Context, *UpsertPoints) (*PointsOperationResponse, error)
// Delete points
Delete(context.Context, *DeletePoints) (*PointsOperationResponse, error)
// Retrieve points
Get(context.Context, *GetPoints) (*GetResponse, error)
// Update named vectors for point
UpdateVectors(context.Context, *UpdatePointVectors) (*PointsOperationResponse, error)
// Delete named vectors for points
DeleteVectors(context.Context, *DeletePointVectors) (*PointsOperationResponse, error)
// Set payload for points
SetPayload(context.Context, *SetPayloadPoints) (*PointsOperationResponse, error)
// Overwrite payload for points
OverwritePayload(context.Context, *SetPayloadPoints) (*PointsOperationResponse, error)
// Delete specified key payload for points
DeletePayload(context.Context, *DeletePayloadPoints) (*PointsOperationResponse, error)
// Remove all payload for specified points
ClearPayload(context.Context, *ClearPayloadPoints) (*PointsOperationResponse, error)

SearchBatch(context.Context, *SearchBatchPoints) (*SearchBatchResponse, error)
// Retrieve closest points based on vector similarity and given filtering conditions
Search(context.Context, *SearchPoints) (*SearchResponse, error)
// Retrieve closest points based on vector similarity and given filtering conditions, grouped by a given field
SearchGroups(context.Context, *SearchPointGroups) (*SearchGroupsResponse, error)
Recommend(context.Context, *RecommendPoints) (*RecommendResponse, error)
Scroll(context.Context, *ScrollPoints) (*ScrollResponse, error)
// Look for the points which are closer to stored positive examples and at the same time further to negative examples.
RecommendBatch(context.Context, *RecommendBatchPoints) (*RecommendBatchResponse, error)
// Look for the points which are closer to stored positive examples and at the same time further to negative examples, grouped by a given field
RecommendGroups(context.Context, *RecommendPointGroups) (*RecommendGroupsResponse, error)
Discover(context.Context, *DiscoverPoints) (*DiscoverResponse, error)
// Batch request points based on { positive, negative } pairs of examples, and/or a target
DiscoverBatch(context.Context, *DiscoverBatchPoints) (*DiscoverBatchResponse, error)
// Count points in collection with given filtering conditions
Count(context.Context, *CountPoints) (*CountResponse, error)
// Perform multiple update operations in one request
UpdateBatch(context.Context, *UpdateBatchPoints) (*UpdateBatchResponse, error)
*/

pub async fn upsert_points(
    host: impl AsRef<str>,
    name: impl AsRef<str>, // 这里已经是collection_name
    req: UpsertPoints,
) -> anyhow::Result<PointsOperationResponse> {
    let url = format!(
        "http://{host}/collections/{name}/points",
        host = host.as_ref(),
        name = name.as_ref(),
    );
    info!("url={:?}", url);
    let client = Client::new();
    let response = client.put(url).json(&req).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let resp = serde_json::from_str::<PointsOperationResponse>(&body)?;
        anyhow::Ok(resp)
    } else {
        let status_code = response.status().as_u16();
        info!(" response status={:?}", status_code);
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            status_code
        ))
    }
}

pub async fn delete_points(
    host: impl AsRef<str>,
    name: impl AsRef<str>, // collection name
    req: DeletePoints,
) -> anyhow::Result<PointsOperationResponse> {
    let client = Client::new();
    let url = format!(
        "http://{host}/collections/{name}/points/delete",
        host = host.as_ref(),
        name = name.as_ref()
    );
    info!("url= {:?}", url);
    let j = serde_json::to_string_pretty(&req);
    info!("request={}", j.unwrap_or_else(|e| e.to_string()));

    let response = client.post(url).json(&req).send().await?;
    if response.status().is_success() {
        let body = response.text().await?;
        let resp = serde_json::from_str::<PointsOperationResponse>(&body)?;
        anyhow::Ok(resp)
    } else {
        let status_code = response.status().as_u16();
        info!(" response status={:?}", status_code);
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            status_code
        ))
    }
}

pub async fn get_points(
    host: impl AsRef<str>,
    name: impl AsRef<str>,
    mut req: GetPoints,
) -> anyhow::Result<GetResponse> {
    let client = Client::new();
    let url = format!(
        "http://{host}/collections/{name}/points",
        host = host.as_ref(),
        name = name.as_ref()
    );
    println!("url= {:?}", url);
    req.with_payload = Some(true);
    req.with_vectors = Some(true);
    let j = serde_json::to_string_pretty(&req);
    println!("request={}", j.unwrap_or_else(|e| e.to_string()));

    let response = client.post(url).json(&req).send().await?;
    if response.status().is_success() {
        let body = response.text().await?;
        let resp = serde_json::from_str::<GetResponse>(&body)?;
        anyhow::Ok(resp)
    } else {
        let status_code = response.status().as_u16();
        info!(" response status={:?}", status_code);
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            status_code
        ))
    }
}

pub async fn count_points(
    host: impl AsRef<str>,
    collection: impl AsRef<str>,
) -> anyhow::Result<CountResponse> {
    let client = Client::new();
    let url = format!(
        "http://{host}/collections/{name}/points/count",
        host = host.as_ref(),
        name = collection.as_ref()
    );

    let response = client.post(url).send().await?;
    if response.status().is_success() {
        let body = response.text().await?;
        let resp = serde_json::from_str::<CountResponse>(&body)?;
        anyhow::Ok(resp)
    } else {
        let status_code = response.status().as_u16();
        info!(" response status={:?}", status_code);
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            status_code
        ))
    }
}

pub async fn search_points(
    host: impl AsRef<str>,
    collection: impl AsRef<str>,
    req: String, // 将请求转化成json进行搜索
) -> anyhow::Result<SearchResponse> {
    let client = Client::new();
    let url = format!(
        "http://{host}/collections/{name}/points/search",
        host = host.as_ref(),
        name = collection.as_ref()
    );
    println!("url= {:?}", url);
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(req)
        .send()
        .await?;
    if response.status().is_success() {
        let resp_body = response.text().await?;
        let resp = serde_json::from_str::<SearchResponse>(&resp_body)?;
        anyhow::Ok(resp)
    } else {
        let status_code = response.status().as_u16();
        info!(" response status={:?}", status_code);
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            status_code
        ))
    }
}

pub async fn recommend_points(
    host: impl AsRef<str>,
    // req: RecommendPoints,
) -> anyhow::Result<RecommendResponse> {
    unimplemented!()
}

pub async fn discover_points(
    host: impl AsRef<str>,
    // req: DiscoverPoints,
) -> anyhow::Result<DiscoverResponse> {
    unimplemented!()
}

mod tests {
    use super::*;
    use std::hash::Hash;
    
    #[tokio::test]
    async fn test_upsert_points() {
        let host = "localhost:6333";
        let name = "teddy_test_col2_20240701214002".to_owned();
        let mut payload_body = HashMap::new(); //<String, Value>
        payload_body.insert(
            "permission".to_owned(),
            Value::Array(vec![
                Value::String("高级经理".to_string()),
                Value::String("技术专家".to_string()),
            ]),
        );
        payload_body.insert("count".to_owned(), Value::Number(Number::from(10)));
        // let p = PointId {
        //     point_id_options: Some(point_id::PointIdOptions::Uuid(
        //         "98077b2a-3ad5-11ef-a451-3e48c2984b36".to_owned(),
        //     )),
        // };
        let point = PointStruct {
            id: Some("1234567890abcdef1234567890abcdef".to_owned()),
            payload: payload_body,
            vector: vec![0.2, -0.6, 0.8, 0.99, 0.22, 0.33, 0.11, 0.5],
        };

        let req = UpsertPoints {
            // collection_name: name.clone(),
            points: vec![point],
            ..Default::default()
        };
        let j = serde_json::to_string_pretty(&req);
        println!("json={}", j.unwrap_or_else(|e| e.to_string()));
        let ret = upsert_points(host, &name, req).await;
        println!("ret = {:?}", ret);
        // tokio::runtime::Builder::new_multi_thread()
        //     .enable_all()
        //     .build()
        //     .unwrap()
        //     .block_on(async {
        //
        //     });
    }
    #[tokio::test]
    async fn test_get_points() {
        let host = "localhost:6333";
        let name = "teddy_test_col2_20240701214002".to_owned();

        let req = GetPoints {
            // collection_name: name.clone(),
            ids: vec!["3383245f-483e-4fa9-a814-d89f68e380cd".to_owned()],
            ..Default::default()
        };
        let j: Result<String, serde_json::Error> = serde_json::to_string_pretty(&req);
        println!("json={}", j.unwrap_or_else(|e| e.to_string()));
        let ret = get_points(host, &name, req).await;
        println!("ret = {:?}", ret);
    }
    #[tokio::test]
    async fn test_delete_points() {
        let host = "localhost:6333";
        let name = "teddy_test_col2_20240701214002".to_owned();
        let _id = Some("1234567890abcdef1234567890abcdef".to_owned());
        let req = DeletePoints {
            points: vec![_id.unwrap()],
            ..Default::default()
        };
        let req_json = serde_json::to_string_pretty(&req);
        println!("json={}", req_json.unwrap_or_else(|e| e.to_string()));
        let ret = delete_points(host, &name, req).await;
        println!("ret={:?}", ret);
    }
    #[tokio::test]
    async fn test_search_points() {
        let host = "localhost:6333";
        let name = "teddy_test_col2_20240701214002".to_owned();
        let tpl_str = r#"  {
            "filter": {
                "must": [
                    { "key": "permission", "match": { "value": "{{ title }}"  } },
                    { "key": "count", "range": { "gte": {{ count }} } }
                ]
            },
            "with_vectors": true,
            "with_payload": true,
            "vector": [ 0.2,-0.6,0.7,0.9,0.2,0.3,0.1,0.5],
            "limit":5
        }
        "#;
        let mut bars_reg = Handlebars::new();
        let mut param_map = HashMap::new();
        param_map.insert("title", Value::String("技术专家".to_owned()));
        param_map.insert("count", Value::Number(Number::from(3)));
        let reg_result = bars_reg.register_template_string("tpl_1", tpl_str);

        let render_str_result = bars_reg.render("tpl_1", &param_map);
        match render_str_result {
            Ok(render_str) => {
                println!("render_str= {}", render_str);
                let resp = search_points(host, name, render_str).await;
                println!("ret={:?}", resp)
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    #[tokio::test]
    async fn test_recommend_points() {}
    #[tokio::test]
    async fn test_discover_points() {}
}
