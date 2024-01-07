use serde::{Deserialize,Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct GcdParameter {
    pub n: u64,
    pub m: u64,
}
