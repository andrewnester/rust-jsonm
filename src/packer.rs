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

#[derive(Default)]
pub struct PackOptions {
    pub pack_string_depth: i32,
    pub no_sequence_id: bool,
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
    dict: HashMap<u64, String>,
    dict_map: HashMap<String, u64>,
    dict_index: u64,
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

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Packer {
    pub fn new() -> Packer {
        Packer {
            sequence_id: -1,
            max_dict_size: 2000,
            dict_index: MIN_DICT_INDEX,
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
                    Ok(mut result) => result,
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
        self.dict = HashMap::new();
        self.dict_map = HashMap::new();
        self.dict_index = MIN_DICT_INDEX;
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

        let mut results: Vec<Value> = Vec::new();
        let obj = match object.as_object() {
            Some(obj) => obj,
            None => {
                return Err(PackerError {
                    cause: "unknown".to_owned(),
                })
            }
        };

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

        Ok(json!(results))
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

        if self.dict_map.contains_key(map_key) {
            let val = self.dict_map.get(map_key);
            return json!(val);
        }

        self.add_to_dict(map_key, str_value);
        if value.is_boolean() || value.is_null() {
            return json!(value);
        }
        if value.is_string() {
            let re = Regex::new(r"^[0-9.]|^~").unwrap();
            if re.is_match(str_value) {
                return json!("~".to_owned() + str_value);
            }
        }

        return json!(str_value);
    }

    fn add_to_dict(&mut self, map_key: &str, str_value: &str) {
        match self.dict.get(&self.dict_index) {
            Some(delete_key) => {
                let key = self.get_map_key_from_str(delete_key);
                self.dict_map.remove(&key)
            }
            None => None,
        };

        self.dict_map.insert(map_key.to_owned(), self.dict_index);
        self.dict.insert(self.dict_index, str_value.to_owned());
        self.dict_index += 1;

        if self.dict_index >= (self.max_dict_size + MIN_DICT_INDEX) {
            self.dict_index = MIN_DICT_INDEX;
        }
    }

    fn get_map_key_from_str(&self, key: &str) -> String {
        let map_key_string = "~".to_owned() + key;
        match key.parse::<u64>() {
            Ok(_parsed) => key.to_owned(),
            Err(_err) => map_key_string,
        }
    }
}
