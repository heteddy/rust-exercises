use crate::pb::svr::ApiError;
use chrono::{DateTime, Local, Utc};
use futures::stream::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, Document},
    options::{self, IndexOptions},
    Collection, IndexModel,
};
use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};
use std::fmt;
use std::{borrow::Borrow, time::Duration};
use tracing::info;

pub trait Entity<Rhs = Self>:  //约束
    fmt::Debug + Clone + Default + de::DeserializeOwned + Serialize
{
    fn update(&mut self, id: Option<ObjectId>, updated_at: DateTime<Utc>);
    fn updating_doc(&self, rhs: &Rhs) -> Document;
}

// 这里必须是模版参数，不能是type的方式
#[feature(async_fn_in_trait)]
pub trait EntityDao<T>
where
    T: Unpin + Send + Sync + Entity,
{
    // type Entity;
    fn col(&self) -> Collection<T>;
    fn indices(&self) -> Vec<IndexModel>;

    async fn create_index(&self) -> Result<(), ApiError> {
        let o = options::CreateIndexOptions::builder()
            .max_time(Duration::from_secs(60))
            .build();
        self.col().create_indexes(self.indices(), o).await?;
        Ok(())
    }

    async fn insert(&self, mut e: T) -> Result<T, ApiError> //mongodb::error::Result<InsertOneResult>
    {
        // let opt = options::InsertOneOptions::build();
        let ret = self.col().insert_one(&e, None).await?;
        info!("inserted id = {:?}", &ret.inserted_id);
        // let mut entity = app.into();
        let _oid = match ret.inserted_id {
            Bson::ObjectId(_id) => Some(_id),
            _ => None,
        };
        let updated_at = Utc::now();
        e.update(_oid, updated_at);
        Ok(e)
    }

    async fn update(&self, _id: impl AsRef<str>, mut e: T) -> Result<T, ApiError> {
        let opt = options::FindOneAndUpdateOptions::builder()
            .upsert(false)
            .build();
        let _id = ObjectId::parse_str(_id)?;
        let updated_at = Utc::now();
        let ret = self
            .col()
            .find_one_and_update(doc! {"_id":_id}, e.updating_doc(&e), opt)
            .await?;

        e.update(Some(_id), updated_at);
        Ok(e)
    }

    async fn get(&self, _id: impl AsRef<str>) -> Result<T, ApiError> {
        let mongo_id = ObjectId::parse_str(_id)?;
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();
        let ret = self.col().find_one(doc! {"_id":mongo_id}, opt).await?;
        info!("find ret :{:?}", ret);
        Ok(ret.unwrap_or_default())
    }

    async fn get_by_name(&self, name: impl AsRef<str>) -> Result<T, ApiError> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();

        let ret = self
            .col()
            .find_one(doc! {"name":name.as_ref(),"deleted_at":0}, opt)
            .await?;
        Ok(ret.unwrap_or_default())
    }

    async fn list(&self, skip: u64, limit: i64) -> Result<Vec<T>, ApiError> {
        let opt = options::FindOptions::builder()
            .sort(doc! {"updated_at":-1})
            .skip(Some(skip))
            .limit(Some(limit))
            .build();

        let mut cursor = self.col().find(doc! {"deleted_at":0}, opt).await?;
        let mut v = Vec::new();
        while let Some(doc) = cursor.next().await {
            if doc.is_ok() {
                v.push(doc.unwrap_or_default());
            }
        }
        Ok(v)
    }
    async fn delete(&self, id: impl AsRef<str>) -> Result<T, ApiError> {
        let opt = options::FindOneAndDeleteOptions::builder().build();
        let oid = ObjectId::parse_str(id)?;
        let ret = self
            .col()
            .find_one_and_delete(doc! {"_id": oid}, opt)
            .await?;
        Ok(ret.unwrap_or_default())
    }
}
