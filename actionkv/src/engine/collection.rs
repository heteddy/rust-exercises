use crate::cache::repo;
use crate::cache::sync;
use crate::dao::base::EntityDao;
use crate::dao::index::{IndexDao, IndexEntity};
use crate::driver::qdrant::collection::{
    create_collection, delete_collection, get_collection, list_aliases as driver_list_aliases,
    list_collection_alias, list_collections, update_alias, update_collection,
};
use crate::driver::qdrant::index;
use crate::pb::engine::qdrant::collection;
use crate::pb::engine::qdrant::collection::{
    ChangeAliases, CollectionOperationResponse, CreateCollection, GetCollectionInfoResponse,
    ListAliasesResponse, ListCollectionsResponse,
};
use crate::pb::engine::search;
use crate::pb::svr::ApiError;
use crate::server::index::IndexSvc;
use chrono::prelude::*;
use std::convert::AsRef;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tracing::{event, info, instrument, span, Level};

#[derive(Clone)]
pub struct CollectionSvc {
    sender: mpsc::Sender<sync::SyncData>,
}

impl CollectionSvc {
    pub fn new(tx: mpsc::Sender<sync::SyncData>) -> Self {
        CollectionSvc { sender: tx }
    }
    // #[instrument(skip_all)]
    // pub fn get_host_address(
    //     &self,
    //     name: impl AsRef<str> + std::fmt::Debug,
    // ) -> Option<String> {
    //     let r = repo::IndexConfigRepo::get_instance();
    //     let i = r.read().unwrap().get_index(name.as_ref());
    //     if let Some(ref e) = i {
    //         info!("get index entity ={:?}", e.name);
    //         let svr_name = e.configure.server
    //     }
    // }
    #[instrument(skip_all)]
    pub async fn list_aliases(&self, name: impl AsRef<str>) -> anyhow::Result<ListAliasesResponse> {
        info!("service list aliases:{:?}", name.as_ref());
        let r = repo::IndexConfigRepo::get_instance();
        // 通过index的name 获取server地址
        let addr = r.read().unwrap().get_svr_http_addrss(name.as_ref());
        info!("list_aliases server address={:?}", &addr);
        if let Some(ref host) = addr {
            driver_list_aliases(host).await
        } else {
            anyhow::Result::Err(anyhow::anyhow!("not found host {}", name.as_ref()))
        }
    }
    #[instrument(skip_all)]
    pub async fn change_alias(
        &self,
        name: impl AsRef<str>,
        req: collection::ChangeAliases,
    ) -> anyhow::Result<CollectionOperationResponse> {
        // let _req = &req;

        let r = repo::IndexConfigRepo::get_instance();
        // 通过index的name 获取server地址
        let addr = r.read().unwrap().get_index_svr_http_address(name.as_ref());
        if let Some(ref host) = addr {
            update_alias(host, req).await
        } else {
            anyhow::Result::Err(anyhow::anyhow!("not found host {:?}", name.as_ref()))
        }
    }
    #[instrument(skip_all)]
    pub async fn get_alias(&self, name: impl AsRef<str>) -> anyhow::Result<ListAliasesResponse> {
        // let _req = &req;

        let r = repo::IndexConfigRepo::get_instance();
        // 通过index的name 获取server地址
        let addr = r.read().unwrap().get_index_svr_http_address(name.as_ref());
        let active_collection = r.read().unwrap().get_active_collection(name.as_ref());
        if active_collection.is_none() {
            anyhow::Result::Err(anyhow::anyhow!(
                "active_collection is None of {:?}",
                name.as_ref()
            ))
        } else {
            if let Some(ref host) = addr {
                list_collection_alias(host, active_collection.unwrap()).await
            } else {
                anyhow::Result::Err(anyhow::anyhow!("not found host {:?}", name.as_ref()))
            }
        }
    }

    #[instrument(skip_all)]
    pub async fn create(
        &self,
        req: search::CollectionReq,
    ) -> anyhow::Result<CollectionOperationResponse> {
        let r = repo::IndexConfigRepo::get_instance();
        let i = r.read().unwrap().get_index(&(req.name));
        if let Some(ref e) = i {
            // 构造创建index的配置参数
            info!("create index {:?}", e.name);
            // 转换成req
            let req: CreateCollection = e.clone().into();
            let svr_name = &e.configure.server;
            let svr_entity = r.read().unwrap().server.get(svr_name);
            let svr_host = match svr_entity {
                Some(host) => host.http_addr.clone(),
                None => String::new(),
            };

            if svr_host.len() == 0 {
                anyhow::Result::Err(anyhow::anyhow!(
                    "not found server svr_name={:?},host={:?}",
                    svr_name,
                    svr_host
                ))
            } else {
                let s = serde_json::to_string_pretty(&req).unwrap();
                info!("collection req={:?}", s);
                // 并非关键路径，可以拷贝
                let ret = create_collection(&svr_host, req).await?; // 创建collection；
                                                                    // 创建index
                let indexes = e.to_field_index_collection();
                // indexes.into_iter().map(|_req|{
                //     index::create_field_index(svr_host, _req).await
                // });
                info!("start to create index len={:?}", indexes.len());
                for _req in indexes.into_iter() {
                    let ret = index::create_field_index(&svr_host, _req).await?;
                    info!("create index:{:?}", ret);
                }

                // TODO: 更新alias，删除原alias，增加inactive到和active

                let mut e2 = e.clone();
                e2.active = e2.inactive.clone();
                let req: ChangeAliases = e2.clone().into();
                info!("updating entity {:?}", e2);

                // 后面还要继续用
                // 1. delete alias
                // 2. create alias
                let req_j = serde_json::to_string_pretty(&req).unwrap();
                info!("updating alias:{:?}", req_j);

                let ret2 = update_alias(&svr_host, req).await?;
                info!("created alias:{:?}", ret2);
                // 更新
                let svc = IndexSvc::new(self.sender.clone());
                svc.update(e2.id.as_ref().unwrap().to_hex(), e2).await;
                info!("updated index");
                anyhow::Ok(ret)
            }
        } else {
            anyhow::Result::Err(anyhow::anyhow!("not found index entity {:?}", req.name))
        }
    }
    #[instrument(skip_all)]
    pub async fn get(
        &self,
        alias: impl AsRef<str> + std::fmt::Debug,
    ) -> anyhow::Result<GetCollectionInfoResponse> {
        // 获取collection的server地址

        let r = repo::IndexConfigRepo::get_instance();
        let addr = r.read().unwrap().get_index_svr_http_address(alias.as_ref());
        let active_collection = r.read().unwrap().get_active_collection(alias.as_ref());

        if active_collection.is_none() {
            anyhow::Result::Err(anyhow::anyhow!(
                "not found active collection {:?}",
                alias.as_ref()
            ))
        } else {
            if let Some(ref svr_host) = addr {
                if svr_host.len() == 0 {
                    anyhow::Result::Err(anyhow::anyhow!(
                        "not found server svr_name={:?},host={:?}",
                        alias,
                        svr_host
                    ))
                } else {
                    // 并非关键路径，可以拷贝

                    let ret = get_collection(svr_host, &(active_collection.unwrap())).await?;
                    anyhow::Ok(ret)
                }
            } else {
                anyhow::Result::Err(anyhow::anyhow!(
                    "not found index entity {:?}",
                    alias.as_ref()
                ))
            }
        }
    }

    #[instrument(skip_all)]
    pub async fn list(&self, svr_name: impl AsRef<str>) -> anyhow::Result<ListCollectionsResponse> {
        let r = repo::IndexConfigRepo::get_instance();
        let svr_entity = r.read().unwrap().server.get(svr_name);

        let svr_host = match svr_entity {
            Some(host) => host.http_addr.clone(),
            None => String::new(),
        };

        if svr_host.len() == 0 {
            anyhow::Result::Err(anyhow::anyhow!("not found server host={:?}", svr_host))
        } else {
            // 并非关键路径，可以拷贝
            let ret = list_collections(&svr_host).await?; // 创建collection；
            anyhow::Ok(ret)
        }
    }

    #[instrument(skip_all)]
    pub async fn delete(
        &self,
        alias: impl AsRef<str> + std::fmt::Debug, // 通过alias 删除？
    ) -> anyhow::Result<CollectionOperationResponse> {
        let r = repo::IndexConfigRepo::get_instance();
        let addr = r.read().unwrap().get_index_svr_http_address(alias.as_ref());

        let active_collection = r.read().unwrap().get_active_collection(alias.as_ref());

        if active_collection.is_none() {
            anyhow::Result::Err(anyhow::anyhow!(
                "not found active collection {:?}",
                alias.as_ref()
            ))
        } else {
            if let Some(ref svr_host) = addr {
                if svr_host.len() == 0 {
                    anyhow::Result::Err(anyhow::anyhow!(
                        "not found server svr_name={:?},host={:?}",
                        alias,
                        svr_host
                    ))
                } else {
                    // 并非关键路径，可以拷贝
                    let ret = delete_collection(svr_host, active_collection.unwrap()).await?;
                    anyhow::Ok(ret)
                }
            } else {
                anyhow::Result::Err(anyhow::anyhow!(
                    "not found index entity {:?}",
                    alias.as_ref()
                ))
            }
        }
    }
}
