# rust-jsonm

[![Build Status](https://travis-ci.org/andrewnester/rust-jsonm.svg?branch=master)](https://travis-ci.org/andrewnester/rust-jsonm)

**jsonm** implementation port for Rust.

Original library written in JS is here: https://github.com/lennartcl/jsonm

> jsonm is a fast and safe way to compress JSON messages using memoization. jsonm makes messages up to several orders of magnitude smaller by getting rid of repeated names and values.

> jsonm packs
```
[
    { "firstName": "Francis", "lastName": "Doe" },
    { "firstName": "Anna", "lastName": "Smith" },
    { "firstName": "Agent", "lastName": "Smith", "isAlias": true },
    { "firstName": "Anna", "lastName": "Francis" }
]
```
> into
```
[ 0,
    ["firstName", "lastName", "Francis", "Doe"],
    [3, 4, "Anna", "Smith"],
    [3, 4, "isAlias", "Agent", 8, true],
    [3, 4, 7, 5]
]
```
> Notice how it eliminates all common substrings like "firstName" using memoization! jsonm keeps a dictionary to compress future messages even further. Send the message above a second time, and it becomes:
```
[0,[3,4,5,6],[3,4,7,8],[3,4,9,10,8,11],[3,4,7,5],1]
```
> And
```
[
    { "firstName": "Bryan", "lastName": "Fuller" },
    { "firstName": "Anna", "lastName": "Adams" },
    { "firstName": "Tim", "lastName": "Peterson" },
    { "firstName": "Francis", "lastName": "Peterson" }
]
```
> becomes
```
[0,[3,4,"Bryan","Fuller"],[3,4,7,"Adams"],[3,4,"Tim","Peterson"],[3,4,5,16]]
```
> By avoiding repetition, jsonm can for example help decrease the size of messages sent from a web server to the client. It effectively leaves out all information the client already knows about.

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
    
