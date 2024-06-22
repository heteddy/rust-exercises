use crate::pb::engine::qdrant::points;
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

/*
// Create index for field in collection
CreateFieldIndex(context.Context, *CreateFieldIndexCollection) (*PointsOperationResponse, error)
// Delete field index for collection
DeleteFieldIndex(context.Context, *DeleteFieldIndexCollection) (*PointsOperationResponse, error)
*/

pub async fn create_field_index(
    host: impl AsRef<str>,
    collection_name: impl AsRef<str>,
    req: points::CreateFieldIndexCollection,
) -> anyhow::Result<points::PointsOperationResponse> {
    let url = format!(
        "http://{host}/collections/{name}/index",
        host = host.as_ref(),
        name = collection_name.as_ref()
    );
    let response = Client::new().put(url).json(&req).send().await?;
    let code = response.status().as_u16();
    if code < 400 {
        let body = response.text().await?; // moved here
        let resp = serde_json::from_str::<points::PointsOperationResponse>(&body)?;
        println!("response received {:?}", resp);
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            code
        ))
    }
}

pub async fn delete_field_index(
    host: impl AsRef<str>,
    collection_name: impl AsRef<str>,
    field_name: impl AsRef<str>,
) -> anyhow::Result<points::PointsOperationResponse> {
    let url = format!(
        "http://{host}/collections/{name}/index/{field_name}",
        host = host.as_ref(),
        name = collection_name.as_ref(),
        field_name = field_name.as_ref()
    );
    let response = Client::new().delete(url).send().await?;
    let code = response.status().as_u16();
    if code < 400 {
        let body = response.text().await?; // moved here
        let resp = serde_json::from_str::<points::PointsOperationResponse>(&body)?;
        println!("response received {:?}", resp);
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            code
        ))
    }
}
