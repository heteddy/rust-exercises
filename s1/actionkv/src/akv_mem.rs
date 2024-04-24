#![cfg_attr(
    debug_assertions,
    allow(
        unused,
        dead_code,
        unused_imports,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]

use ansi_term::Colour;
use clap::{Parser, Subcommand};
use config::{cc, kafka,configure};
use lazy_static::lazy_static;
use log::{info, warn};
use serde_yaml;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use tokio::sync::mpsc;
// use std::sync::t;

use libakv::config::{self, logger, mongo};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use std::time::Duration;



#[tokio::main]
async fn main() {
    thread::sleep(Duration::from_secs(2));
    logger::setup_logger(true, None);
    {
        let mut v = cc::GLOBAL_CONFIG.lock().unwrap();
        v.mongo.replica = "rs1".to_owned();
        info!("configure ={:?}", v);
    }
    
    configure::init_configure().await;
}
