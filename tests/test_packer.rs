#[macro_use]
extern crate serde_json;
extern crate jsonm;

const TYPE_ARRAY: u32 = 0;
const TYPE_VALUE: u32 = 1;
const TYPE_STRING: u32 = 2;

use jsonm::packer::{PackOptions, Packer};
use jsonm::unpacker::Unpacker;
use serde_json::Value;

#[test]
fn it_packs_small_ints_as_string_values() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!({"foo": 1}), &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(packed, json!(["foo", "1", 0]));
    assert_eq!(unpacked, json!({"foo": 1}));
}

#[test]
fn it_packs_large_ints_as_string_values() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!({"foo": 1000}), &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(packed, json!(["foo", "1000", 0]));
    assert_eq!(unpacked, json!({"foo": 1000}));
}

#[test]
fn it_packs_arrays_using_minus_1() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!([0, 1, 2]), &options).unwrap();
    assert_eq!(packed, json!([TYPE_ARRAY, "0", "1", "2", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([0, 1, 2]));
}

#[test]
fn it_packs_floats_just_fine() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!(1.5), &options).unwrap();
    assert_eq!(packed, json!([TYPE_VALUE, "1.5", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(1.5));
}

#[test]
fn it_packs_true_just_fine() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!(true), &options).unwrap();
    assert_eq!(packed, json!([TYPE_VALUE, true, 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(true));
}

#[test]
fn it_packs_arrays_with_minus_1_just_fine() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!([-1]), &options).unwrap();
    assert_eq!(packed, json!([TYPE_ARRAY, "-1", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([-1]));
}

#[test]
fn it_packs_empty_arrays_just_fine() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!([]), &options).unwrap();
    assert_eq!(packed, json!([TYPE_ARRAY, 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([]));
}

#[test]
fn it_packs_number_string_just_fine() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!("1"), &options).unwrap();
    assert_eq!(packed, json!([TYPE_VALUE, "~1", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!("1"));
}

#[test]
fn it_packs_dot_number_string_just_fine() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!(".1"), &options).unwrap();
    assert_eq!(packed, json!([TYPE_VALUE, "~.1", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(".1"));
}

#[test]
fn it_packs_tilde_strings_just_fine() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!("~1"), &options).unwrap();
    assert_eq!(packed, json!([TYPE_VALUE, "~~1", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!("~1"));
}

#[test]
fn it_packs_tilde_tilde_strings_just_fine() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!("~~1"), &options).unwrap();
    assert_eq!(packed, json!([TYPE_VALUE, "~~~1", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!("~~1"));
}

#[test]
fn it_packs_multi_key_objects() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer
        .pack(
            &json!( { "bar": 1, "baz": 2, "foo": { "qux": 3 } }),
            &options,
        ).unwrap();
    assert_eq!(
        packed,
        json!(["bar", "baz", "foo", "1", "2", ["qux", "3"], 0])
    );

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "bar": 1, "baz": 2, "foo": { "qux": 3 } }));
}

#[test]
fn it_packs_nested_arrays_and_objects() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!([{"a":[[{"b":12}]]}]), &options).unwrap();
    assert_eq!(
        packed,
        json!([
            TYPE_ARRAY,
            ["a", [TYPE_ARRAY, [TYPE_ARRAY, ["b", "12"]]]],
            0
        ])
    );

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([{"a":[[{"b":12}]]}]));
}

#[test]
fn it_uses_memoization_the_second_time() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer
        .pack(&json!({ "bar": 1, "foo": 2 }), &options)
        .unwrap();
    assert_eq!(packed, json!(["bar", "foo", "1", "2", 0]));
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "bar": 1, "foo": 2 }));

    let packed = packer
        .pack(&json!({ "bar": 1, "foo": 2 }), &options)
        .unwrap();
    assert_eq!(packed, json!([3, 4, 5, 6, 1]));
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "bar": 1, "foo": 2 }));

    let packed = packer
        .pack(&json!({ "bar": 1, "foo": 2 }), &options)
        .unwrap();
    assert_eq!(packed, json!([3, 4, 5, 6, 2]));
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "bar": 1, "foo": 2 }));
}

#[test]
fn it_handles_strings_and_ints_with_the_same_value() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer
        .pack(&json!({ "bar": 1, "foo": "1" }), &options)
        .unwrap();
    assert_eq!(packed, json!(["bar", "foo", "1", "~1", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "bar": 1, "foo": "1" }));
}

#[test]
fn it_packs_multi_line_strings_as_normal_values() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer
        .pack(
            &"hello there\nthis is\r\na multi-line string".to_owned(),
            &options,
        ).unwrap();
    assert_eq!(
        packed,
        json!([TYPE_VALUE, "hello there\nthis is\r\na multi-line string", 0])
    );

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, "hello there\nthis is\r\na multi-line string");
}

#[test]
fn it_packs_multi_line_strings_as_separate_values_in_string_packing_mode() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer
        .pack_string(
            &"hello there\nthis is\r\na multi-line string".to_owned(),
            &options,
        ).unwrap();
    assert_eq!(
        packed,
        json!([
            TYPE_STRING,
            "hello there",
            "this is",
            "a multi-line string",
            0
        ])
    );
}

#[test]
fn it_packs_json_strings() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer
        .pack_string(&"{\"bar\":1,\"foo\":2}".to_owned(), &options)
        .unwrap();
    assert_eq!(packed, json!(["bar", "foo", "1", "2", 0]));
}

#[test]
fn it_has_no_issues_going_over_a_very_small_dictionary_size() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();
    packer.set_max_dict_size(6);
    unpacker.set_max_dict_size(6);

    let options = PackOptions::new();
    let packed = packer.pack(&[1, 2, 3, 4], &options).unwrap();
    let unpacked: Vec<i32> = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, [1, 2, 3, 4]);

    let packed = packer.pack(&[7, 8, 1, 2], &options).unwrap();
    assert_eq!(packed, json!([TYPE_ARRAY, "7", "8", 3, 4, 1]));
    let unpacked: Vec<i32> = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, [7, 8, 1, 2]);

    let packed = packer.pack(&[1, 2, 5, 6, 1, 5], &options).unwrap();
    assert_eq!(packed, json!([TYPE_ARRAY, 3, 4, "5", "6", "1", 3, 2]));
    let unpacked: Vec<i32> = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, [1, 2, 5, 6, 1, 5]);

    let packed = packer
        .pack(&json!({ "5": 11, "10": 5, "12": 13 }), &options)
        .unwrap();
    assert_eq!(packed, json!(["~10", "~12", "~5", 3, "13", "11", 3]));
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "5": 11, "10": 5, "12": 13 }));

    let packed = packer
        .pack(&json!({ "5": 11, "10": 5, "12": 14 }), &options)
        .unwrap();
    assert_eq!(packed, json!([6, 7, 8, "5", "14", 4, 4]));
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "5": 11, "10": 5, "12": 14 }));
}
