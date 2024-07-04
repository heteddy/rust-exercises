use crate::pb::engine::qdrant::points::{
    vectors, CountResponse, DeletePoints, DiscoverBatchResponse, DiscoverResponse, GetPoints,
    GetResponse, PointsOperationResponse, RecommendBatchResponse, RecommendGroupsResponse,
    RecommendResponse, RetrievedPoint, ScoredPoint, ScrollResponse, SearchBatchResponse,
    SearchResponse, UpdateBatchResponse, UpsertPoints,
};
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

pub async fn upsert(
    host: impl AsRef<str>,
    req: UpsertPoints,
) -> anyhow::Result<PointsOperationResponse> {
    unimplemented!()
}

pub async fn delete(
    host: impl AsRef<str>,
    req: DeletePoints,
) -> anyhow::Result<PointsOperationResponse> {
    unimplemented!()
}

pub async fn get(host: impl AsRef<str>, req: GetPoints) -> anyhow::Result<GetResponse> {
    unimplemented!()
}

pub async fn search(
    host: impl AsRef<str>,
    // req: SearchPoints,
) -> anyhow::Result<SearchResponse> {
    unimplemented!()
}

pub async fn recommend(
    host: impl AsRef<str>,
    // req: RecommendPoints,
) -> anyhow::Result<RecommendResponse> {
    unimplemented!()
}

pub async fn discover(
    host: impl AsRef<str>,
    // req: DiscoverPoints,
) -> anyhow::Result<DiscoverResponse> {
    unimplemented!()
}

mod tests {
    use super::*;
    #[test]
    fn test_upsert() {}
    #[test]
    fn test_get() {}
    #[test]
    fn test_delete() {}
    #[test]
    fn test_search() {}
    #[test]
    fn test_recommend() {}
    #[test]
    fn test_discover() {}
}
