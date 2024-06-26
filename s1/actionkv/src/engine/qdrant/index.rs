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
use tracing::{info, instrument, warn};

/*
// Create index for field in collection
CreateFieldIndex(context.Context, *CreateFieldIndexCollection) (*PointsOperationResponse, error)
// Delete field index for collection
DeleteFieldIndex(context.Context, *DeleteFieldIndexCollection) (*PointsOperationResponse, error)
*/

/*
keyword - for keyword payload, affects Match filtering conditions.
integer - for integer payload, affects Match and Range filtering conditions.
float - for float payload, affects Range filtering conditions.
bool - for bool payload, affects Match filtering conditions (available as of v1.4.0).
geo - for geo payload, affects Geo Bounding Box and Geo Radius filtering conditions.
datetime - for datetime payload, affects Range filtering conditions (available as of v1.8.0).
text - a special kind of index, available for keyword / string payloads, affects Full Text search filtering conditions.


PUT /collections/{collection_name}/index
{
    "field_name": "name_of_the_field_to_index",
    "field_schema": "keyword"
}

PUT /collections/{collection_name}/index
{
    "field_name": "name_of_the_field_to_index",
    "field_schema": {
        "type": "text",
        "tokenizer": "word",
        "min_token_len": 2,
        "max_token_len": 20,
        "lowercase": true
    }
}
*/

/// create
#[instrument(skip_all)]
pub async fn create_field_index(
    host: impl AsRef<str>,
    // collection_name: impl AsRef<str>,
    req: points::CreateFieldIndexCollection,
) -> anyhow::Result<points::PointsOperationResponse> {
    let url = format!(
        "http://{host}/collections/{name}/index",
        host = host.as_ref(),
        name = req.collection_name.clone(),
    );
    info!(
        "create_field_index url={:?}, req={:?}",
        url,
        serde_json::to_string_pretty(&req).unwrap()
    );

    let response = Client::new().put(url).json(&req).send().await?;
    let code = response.status().as_u16();
    info!("create_field_index status code = {:?}", code);
    if code < 400 {
        let body = response.text().await?; // moved here
        let resp = serde_json::from_str::<points::PointsOperationResponse>(&body)?;
        info!("create_field_index response received {:?}", resp);
        anyhow::Ok(resp)
    } else {
        anyhow::Result::Err(anyhow::anyhow!(
            "engine request error; status code = {:?}",
            code
        ))
    }
}
#[instrument(skip_all)]
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
