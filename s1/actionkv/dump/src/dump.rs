use bincode::de;
use bincode::deserialize as from_bincode;
use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::{self, to_string as to_json};

use crate::data;


pub fn dump() {
    let c = data::City::new("成都", 21000000, 10.98, 123.99);

    let as_json = to_json(&c).unwrap();
    let as_cbor = to_cbor(&c).unwrap();
    let as_bincode = to_bincode(&c).unwrap();

    println!("json:\n\t{},\n\tlen:{}\n", as_json, as_json.len()); //调用display
    println!("cbor:\n\t{:?} \n\tlen={}\n", as_cbor, as_cbor.len());
    println!(
        "bincode:\n\t{:?} \n\tlen={}\n",
        as_bincode,
        as_bincode.len()
    );
    // 从vec转变到切片
    let b = &as_bincode[..];

    let c2: data::City = serde_json::from_str(&as_json).unwrap();
    let c3: data::City = from_bincode(&as_bincode[..]).unwrap();
    println!("c2={:?}", c2);
    println!("c3={:?}", c3);
    println!("Hello, world!");
}
