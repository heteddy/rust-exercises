use anyhow;
use http::Method;
use lazy_static::lazy_static;
use reqwest::{Client, ClientBuilder, Request, Url};
use std::collections::HashMap;
use std::time::Duration;

static LIST_COLLECTION: &str = "LIST_COLLECTION";

lazy_static! {
    pub static ref COLLECTION_CONFIGURE: HashMap<&'static str, QdrantAccess> = init_collection_op();
}

#[derive(Clone, PartialEq)]
pub struct QdrantAccess {
    method: http::Method,
    url_pattern: String,
}

fn init_collection_op() -> HashMap<&'static str, QdrantAccess> {
    let mut map = HashMap::with_capacity(10);
    map.insert(
        LIST_COLLECTION,
        QdrantAccess {
            method: http::Method::GET,
            url_pattern: String::from("http://{host}/collections/{collection_name}"),
        },
    );
    map
}

pub async fn list_collections(host: impl AsRef<str>) -> anyhow::Result<()> {
    let url = format!("http://{host}/collections", host = host.as_ref());
    let client = Client::new();
    // let client = Client::builder()
    //     .retry(|attempt| {
    //         let url = Url::parse(&url).unwrap();
    //         if attempt > 3 {
    //             return None;
    //         }
    //         Some(Duration::from_secs(attempt * 2) + url.host_str().unwrap().len() as u64)
    //     })
    //     .build()?;
    let response = client.get(&url).send().await.unwrap();
    println!("{}", response.text().await.unwrap());
    anyhow::Ok(())
}

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
