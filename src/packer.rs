extern crate regex;
extern crate serde;
extern crate serde_json;

use self::regex::Regex;
use self::serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::vec::Vec;

const MIN_DICT_INDEX: u64 = 3;
const TYPE_ARRAY: u32 = 0;
const TYPE_VALUE: u32 = 1;
const TYPE_STRING: u32 = 2;
const MAX_PACK_COMPLEX_OBJECT_SIZE: usize = 12;

#[derive(Default)]
pub struct PackOptions {
    pub pack_string_depth: i32,
    pub no_sequence_id: bool,
}

#[derive(Default, Debug)]
pub struct MemoObject {
    pub key: String,
    pub value: String,
}

impl PackOptions {
    pub fn new() -> PackOptions {
        PackOptions {
            pack_string_depth: -1,
            no_sequence_id: false,
        }
    }
}

/// Packer used to pack/compress json-like structures.
#[derive(Default, Debug)]
pub struct Packer {
    memoised: HashMap<u64, MemoObject>,
    memoised_map: HashMap<String, u64>,
    memoised_object_map: HashMap<String, u64>,
    memoised_index: u64,
    sequence_id: i64,
    max_dict_size: u64,
}

#[derive(Debug, Clone)]
pub struct PackerError {
    cause: String,
}

impl fmt::Display for PackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PackerError")
    }
}

impl Error for PackerError {
    fn description(&self) -> &str {
        "Packer Error"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Packer {
    pub fn new() -> Packer {
        Packer {
            sequence_id: -1,
            max_dict_size: 2000,
            memoised_index: MIN_DICT_INDEX,
            ..Default::default()
        }
    }

    /// Pack an JSON-like object.
    pub fn pack<T>(&mut self, object: &T, options: &PackOptions) -> Result<Value, PackerError>
    where
        T: Serialize,
    {
        let json_object = json!(object);
        let result = self.pack_object_or_value(&json_object, options.pack_string_depth);
        if options.no_sequence_id {
            return result;
        }

        match result {
            Ok(mut value) => {
                self.sequence_id += 1;
                if !value.is_array() {
                    return Ok(json!([json!(TYPE_VALUE), value, json!(self.sequence_id)]));
                }

                match value.as_array_mut() {
                    Some(result) => {
                        result.push(json!(self.sequence_id));
                        Ok(json!(result))
                    }
                    None => Err(PackerError {
                        cause: "unknown".to_owned(),
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Pack a string. Efficiently packs multi-line strings and JSON strings.
    /// When unpacked, a string is always returned again.
    pub fn pack_string(
        &mut self,
        string_to_pack: &str,
        options: &PackOptions,
    ) -> Result<Value, PackerError> {
        match serde_json::from_str(string_to_pack) {
            Ok(value) => self.pack::<Value>(&value, options),
            Err(_err) => {
                let string = string_to_pack.to_owned();
                let lines = string.lines();
                let mut result_vec = Vec::new();
                for line in lines {
                    result_vec.push(json!(line));
                }

                let mut result = match self.pack(&json!(result_vec), options) {
                    Ok(result) => result,
                    Err(err) => return Err(err),
                };

                let vec = match result.as_array_mut() {
                    Some(result) => result,
                    None => {
                        return Err(PackerError {
                            cause: "unknown".to_owned(),
                        })
                    }
                };

                vec[0] = json!(TYPE_STRING);
                Ok(json!(vec))
            }
        }
    }
    /// Reset the memoization dictionary, allowing consumption by new Unpacker instances.
    pub fn reset(&mut self) {
        self.memoised = HashMap::new();
        self.memoised_map = HashMap::new();
        self.memoised_object_map = HashMap::new();
        self.memoised_index = MIN_DICT_INDEX;
        self.sequence_id = -1;
    }

    /// Set the maximum dictionary size. Must match the dictionary size used by the unpacker.
    /// Default - 2000
    pub fn set_max_dict_size(&mut self, value: u64) {
        self.max_dict_size = value;
    }

    fn pack_object_or_value(
        &mut self,
        object: &Value,
        pack_string_depth: i32,
    ) -> Result<Value, PackerError> {
        if object.is_null() {
            return Ok(Value::Null);
        }

        if object.is_array() {
            let arr = match object.as_array() {
                Some(arr) => arr,
                None => {
                    return Err(PackerError {
                        cause: "unknown".to_string(),
                    })
                }
            };
            return Ok(self.pack_array(arr, pack_string_depth - 1));
        }

        if object.is_string() && pack_string_depth >= 0 {
            let obj_str = match object.as_str() {
                Some(arr) => arr,
                None => {
                    return Err(PackerError {
                        cause: "unknown".to_string(),
                    })
                }
            };
            let options = PackOptions {
                no_sequence_id: true,
                pack_string_depth: 0,
            };
            return self.pack_string(obj_str, &options);
        }

        if !object.is_object() {
            return Ok(self.pack_value(object));
        }

        return self.pack_object(object, pack_string_depth);
    }

    fn pack_object(
        &mut self,
        object: &Value,
        pack_string_depth: i32,
    ) -> Result<Value, PackerError> {
        let obj = match object.as_object() {
            Some(obj) => obj,
            None => {
                return Err(PackerError {
                    cause: "unknown".to_owned(),
                })
            }
        };
        let mut results: Vec<Value> = Vec::new();
        for (key, _value) in obj {
            results.push(self.pack_value(&json!(key)));
        }

        for (_key, value) in obj {
            if value.is_object() || value.is_array() {
                match self.pack_object_or_value(value, pack_string_depth - 1) {
                    Ok(object) => results.push(object),
                    Err(err) => return Err(err),
                };
            } else if value.is_string() {
                if pack_string_depth > 0 {
                    let string = match value.as_str() {
                        Some(s) => s,
                        None => {
                            return Err(PackerError {
                                cause: "unknown".to_owned(),
                            })
                        }
                    };
                    let packed_string = match self.pack_string(
                        string,
                        &PackOptions {
                            no_sequence_id: true,
                            pack_string_depth: -1,
                        },
                    ) {
                        Ok(packed_string) => packed_string,
                        Err(err) => return Err(err),
                    };
                    results.push(packed_string);
                } else {
                    results.push(self.pack_value(value));
                }
            } else {
                results.push(self.pack_value(value));
            }
        }

        return Ok(self.try_pack_complex_object(object, results));
    }

    fn try_pack_complex_object(&mut self, object: &Value, results: Vec<Value>) -> Value {
        if results.len() > MAX_PACK_COMPLEX_OBJECT_SIZE {
            return json!(results);
        }

        for v in &results {
            if !v.is_number() {
                return json!(results);
            }
        }

        let key = object.to_string();
        if self.memoised_object_map.contains_key(&key) {
            let val = self.memoised_object_map.get(&key);
            return json!(val);
        }

        self.memoise(&object.to_string(), &key, true);

        return json!(results);
    }

    fn pack_array(&mut self, object: &[Value], pack_string_depth: i32) -> Value {
        let mut result: Vec<Value> = Vec::new();
        result.push(json!(TYPE_ARRAY));
        for val in object {
            match self.pack_object_or_value(val, pack_string_depth) {
                Ok(packed_object) => result.push(packed_object),
                Err(_err) => {}
            }
        }

        json!(result)
    }

    fn pack_value(&mut self, value: &Value) -> Value {
        let string = value.to_string();
        let str_value: &str = match value.as_str() {
            Some(v) => v,
            None => string.as_str(),
        };

        let map_key_string = "~".to_owned() + str_value;
        let map_key = if value.is_string() {
            map_key_string.as_str()
        } else {
            str_value
        };

        if self.memoised_map.contains_key(map_key) {
            let val = self.memoised_map.get(map_key);
            return json!(val);
        }

        if value.is_boolean() || value.is_null() {
            self.memoise(str_value, map_key, false);
            return json!(value);
        }

        if value.is_number() {
            self.memoise(str_value, map_key, false);
            return json!(str_value);
        }

        if value.is_string() {
            self.memoise(str_value, map_key, false);
            let re = Regex::new(r"^[0-9.]|^~").unwrap();
            if re.is_match(str_value) {
                return json!("~".to_owned() + str_value);
            }
        }

        return json!(str_value);
    }

    fn memoise(&mut self, str_value: &str, map_key: &str, is_object: bool) {
        match self.memoised.get(&self.memoised_index) {
            Some(found_object) => {
                let key = &found_object.key;
                self.memoised_map.remove(key);
                self.memoised_object_map.remove(key);
            }
            None => (),
        }

        if is_object {
            self.memoised_object_map
                .insert(map_key.to_owned(), self.memoised_index);
        } else {
            self.memoised_map
                .insert(map_key.to_owned(), self.memoised_index);
        }

        self.memoised.insert(
            self.memoised_index,
            MemoObject {
                key: map_key.to_owned(),
                value: str_value.to_owned(),
            },
        );
        self.memoised_index += 1;

        if self.memoised_index >= (self.max_dict_size + MIN_DICT_INDEX) {
            self.memoised_index = MIN_DICT_INDEX;
        }
    }
}
