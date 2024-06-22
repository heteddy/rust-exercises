
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
// Create index for field in collection
CreateFieldIndex(context.Context, *CreateFieldIndexCollection) (*PointsOperationResponse, error)
// Delete field index for collection
DeleteFieldIndex(context.Context, *DeleteFieldIndexCollection) (*PointsOperationResponse, error)
*/