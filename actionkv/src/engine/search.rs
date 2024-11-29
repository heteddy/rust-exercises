use crate::cache::repo;
use crate::cache::sync;
use crate::dao::base::EntityDao;
use crate::dao::index::{IndexDao, IndexEntity};
use crate::driver::qdrant::index;
use crate::driver::qdrant::points as driver_points;
use crate::pb::engine::qdrant::collection as pb_collection;
use crate::pb::engine::qdrant::points as pb_points;
use crate::pb::engine::qdrant::points::SearchResponse;
use crate::pb::engine::search;
use crate::pb::svr::ApiError;
use crate::server::index::IndexSvc;
use chrono::prelude::*;
use handlebars::{self, Handlebars};
use serde_json::{self, json, Map, Number, Value};
use std::collections::HashMap;
use std::convert::AsRef;
use std::fmt::Debug;
use tokio::sync::mpsc;
use std::cell::RefCell;
use tracing::{event, info, instrument, span, Level};

#[derive(Clone)]
pub struct SearchSvc<'a> {
    bars_reg: Handlebars<'a>, //需要一个reg，怎么定义
}

impl<'a> SearchSvc<'a> {
    pub fn new() -> Self {
        let mut bars_reg = Handlebars::new();
        bars_reg.register_helper(
            "stringify",
            Box::new(
                |h: &handlebars::Helper,
                 r: &Handlebars,
                 _: &handlebars::Context,
                 rc: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output|
                 -> handlebars::HelperResult {

                    // if r.strict_mode() && x.is_value_missing() {
                    //     None
                    // } else {
                    //     Some(x.value())
                    // }
                    let param =
                        h.param(0)
                            .ok_or(handlebars::RenderErrorReason::ParamNotFoundForIndex(
                                "closure-helper",
                                0,
                            ))?;
                    println!("{:?}", &param);
                    // out.write("3rd helper: ")?;
                    // let v = param.value();
                    let v_result = serde_json::to_string_pretty(param.value());
                    let v = v_result.unwrap();

                    out.write(&v)?;
                    Ok(())
                },
            ),
        );
        SearchSvc { bars_reg: bars_reg }
    }
    #[instrument(skip_all)]
    pub async fn search(
        &self,
        name: String,
        template: String,
        params: HashMap<String, Value>,
    ) -> anyhow::Result<pb_points::SearchResponse> {
        info!("search svc running...");
        let r = repo::IndexConfigRepo::get_instance();
        let host = r.read().unwrap().get_index_svr_http_address(&name);
        if host.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found host {}", &name));
        }
        let active = r.read().unwrap().get_active_collection(&name);
        if active.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found active collection {}", &name));
        }
        let tpl_body = r.read().unwrap().get_template_body(&template);
        if tpl_body.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found template {}", &template));
        }
        info!("handle bar template parsing");
        // let mut bars_reg = Handlebars::new();
        // bars_reg.register_helper(
        //     "stringify", //增加的名称
        //     Box::new(
        //         |h: &handlebars::Helper,
        //          r: &Handlebars,
        //          _: &handlebars::Context,
        //          rc: &mut handlebars::RenderContext,
        //          out: &mut dyn handlebars::Output|
        //          -> handlebars::HelperResult {
        //             let param =
        //                 h.param(0)
        //                     .ok_or(handlebars::RenderErrorReason::ParamNotFoundForIndex(
        //                         "closure-helper",
        //                         0,
        //                     ))?;
        //             println!("{:?}", &param);
        //             // out.write("3rd helper: ")?;
        //             // let v = param.value();
        //             let v_result = serde_json::to_string_pretty(param.value());
        //             let v = v_result.unwrap();

        //             out.write(&v)?;
        //             Ok(())
        //         },
        //     ),
        // );
        let mut reg_bars2 = self.bars_reg.clone();
        let reg_result = reg_bars2.register_template_string(&name, tpl_body.unwrap());
        match reg_result {
            Err(e) => anyhow::Result::Err(anyhow::anyhow!(
                "not found register template error {}",
                e.to_string()
            )),
            _ => {
                info!("register success {:?}", params);
                let render_str_result = reg_bars2.render(&name, &params);
                match render_str_result {
                    Ok(render_str) => {
                        println!("render_str= {}", render_str);
                        driver_points::search_points(host.unwrap(), active.unwrap(), render_str)
                            .await
                        // anyhow::Result::Ok(resp)
                        // anyhow::Result::Ok(SearchResponse::default())
                    }
                    Err(e) => anyhow::Result::Err(anyhow::anyhow!(
                        "render template error;detail= {}",
                        e.to_string()
                    )),
                }
            }
        }
    }
}
