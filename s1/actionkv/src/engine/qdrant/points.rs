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
