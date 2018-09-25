# rust-jsonm

**jsonm** implementation port for Rust.

Original library written in JS is here: https://github.com/lennartcl/jsonm

> jsonm is a fast and safe way to compress JSON messages using memoization. jsonm makes messages up to several orders of magnitude smaller by getting rid of repeated names and values.

## Installation

    [dependencies]
    "jsonm" = "0.1"

## Examples
    
    #[macro_use]
    extern crate serde_json;
    extern crate jsonm;
    
    use jsonm::packer::{PackOptions, Packer};
    use jsonm::unpacker::Unpacker;
    
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!({ "bar": 1, "foo": "1" }), &options).unwrap(); // packed is ["bar", "foo", "1", "~1", 0]
    
    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap(); // unpacked is Object({ "bar": 1, "foo": "1" })
    
