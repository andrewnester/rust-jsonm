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
fn it_packs_small_integers_as_string_values() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!({"foo": 1}), &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(packed, json!(["foo", "1", 0]));
    assert_eq!(unpacked, json!({"foo": 1}));
}

#[test]
fn it_packs_large_integers_as_string_values() {
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
        )
        .unwrap();
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
        )
        .unwrap();
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
        )
        .unwrap();
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

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, "hello there\nthis is\na multi-line string");
}

#[test]
fn it_copes_with_calling_unpack_multiple_times() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack(&json!({"foo": 1}), &options).unwrap();
    assert_eq!(packed, json!(["foo", "1", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({"foo": 1}));

    match unpacker.unpack(&packed) {
        Ok(s) => s,
        Err(err) => assert_eq!(err.cause, "unable to unpack to specific type"),
    };
}

#[test]
fn it_packs_json_strings() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer
        .pack_string(&"{\"bar\":1,\"foo\":2}".to_owned(), &options)
        .unwrap();
    assert_eq!(packed, json!(["bar", "foo", "1", "2", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({"bar": 1, "foo": 2}));
}

#[test]
fn it_packs_json_strings_and_can_unpack_them_as_string() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer
        .pack_string(&"{\"bar\":1,\"foo\":2}".to_owned(), &options)
        .unwrap();
    assert_eq!(packed, json!(["bar", "foo", "1", "2", 0]));

    let mut unpacker = Unpacker::new();
    let unpacked: String = unpacker.unpack_string(&packed).unwrap();
    assert!(["{\"bar\":1,\"foo\":2}", "{\"foo\":2,\"bar\":1}"].contains(&unpacked.as_str()));
}

#[test]
fn it_has_symmetrical_pack_string_and_unpack_string_for_strings() {
    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer
        .pack_string(
            &"hello there\nthis is\na multi-line string".to_owned(),
            &options,
        )
        .unwrap();
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

    let mut unpacker = Unpacker::new();
    let unpacked: String = unpacker.unpack_string(&packed).unwrap();
    assert_eq!(unpacked, "hello there\nthis is\na multi-line string");
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
    assert_eq!(packed, json!(["~5", "~10", "~12", "11", "5", "13", 3]));
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "5": 11, "10": 5, "12": 13 }));

    let packed = packer
        .pack(&json!({ "5": 11, "10": 5, "12": 14 }), &options)
        .unwrap();
    assert_eq!(packed, json!([6, 7, 8, 3, 4, "14", 4]));
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "5": 11, "10": 5, "12": 14 }));
}

#[test]
fn it_has_no_issues_going_over_dictionary_size() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();
    packer.set_max_dict_size(99);
    unpacker.set_max_dict_size(99);

    let options = PackOptions::new();
    for i in 0..500 {
        let packed = packer
            .pack(
                &json!({"bar": 50, "baz": 60, "foo": i, "qux": i + 1}),
                &options,
            )
            .unwrap();
        let unpacked: Value = unpacker.unpack(&packed).unwrap();
        assert_eq!(
            unpacked,
            json!({"bar": 50, "baz": 60, "foo": i, "qux": i + 1})
        );
    }
}

#[test]
fn it_handles_packer_being_reset() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    for _i in 0..100 {
        let packed = packer
            .pack(&json!({"foo": 50, "bar": 60}), &options)
            .unwrap();
        let unpacked: Value = unpacker.unpack(&packed).unwrap();
        assert_eq!(unpacked, json!({"foo": 50, "bar": 60}));
    }

    let mut packer = Packer::new();
    for _i in 0..100 {
        let packed = packer
            .pack(&json!({"foo": 50, "bar": 60}), &options)
            .unwrap();
        let unpacked: Value = unpacker.unpack(&packed).unwrap();
        assert_eq!(unpacked, json!({"foo": 50, "bar": 60}));
    }
}

#[test]
fn it_errors_when_the_packer_is_reset_during_message() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer
        .pack(&json!({"foo": 50, "bar": 60}), &options)
        .unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({"foo": 50, "bar": 60}));

    packer
        .pack(&json!({"foo": 50, "bar": 60}), &options)
        .unwrap();
    let mut unpacker = Unpacker::new();
    match unpacker.unpack(&packed) {
        Ok(s) => s,
        Err(err) => assert_eq!(err.cause, "unable to unpack to specific type"),
    };
}

#[test]
fn it_handles_small_messages_with_more_values_than_dictionary_size() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    packer.set_max_dict_size(5);
    unpacker.set_max_dict_size(5);

    let options = PackOptions::new();
    let packed = packer
        .pack(&json!([1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 6]), &options)
        .unwrap();
    assert_eq!(
        packed,
        json!([TYPE_ARRAY, "1", "2", "3", "4", "5", 3, 4, 5, 6, 7, "6", 0])
    );

    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 6]));

    let packed = packer
        .pack(&json!([1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 6]), &options)
        .unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 6]));
}

#[test]
fn it_handles_small_messages_with_more_values_than_dictionary_size_2() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    packer.set_max_dict_size(4);
    unpacker.set_max_dict_size(4);

    let options = PackOptions::new();
    let packed = packer
        .pack(&json!([1, 2, 3, 4, 1, 2, 3, 4, 1]), &options)
        .unwrap();
    assert_eq!(
        packed,
        json!([TYPE_ARRAY, "1", "2", "3", "4", 3, 4, 5, 6, 3, 0])
    );

    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([1, 2, 3, 4, 1, 2, 3, 4, 1]));

    let packed = packer
        .pack(&json!([1, 2, 3, 4, 1, 2, 3, 4, 1]), &options)
        .unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([1, 2, 3, 4, 1, 2, 3, 4, 1]));
}

#[test]
fn it_handles_small_messages_with_more_values_than_dictionary_size_3() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    packer.set_max_dict_size(3);
    unpacker.set_max_dict_size(3);

    let options = PackOptions::new();
    let packed = packer
        .pack(&json!([1, 2, 3, 4, 1, 2, 3, 4, 1]), &options)
        .unwrap();

    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([1, 2, 3, 4, 1, 2, 3, 4, 1]));

    let packed = packer
        .pack(&json!([1, 2, 3, 4, 1, 2, 3, 4, 1]), &options)
        .unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([1, 2, 3, 4, 1, 2, 3, 4, 1]));
}

#[test]
fn it_handles_large_messages_with_more_values_than_dictionary_size() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    packer.set_max_dict_size(50);
    unpacker.set_max_dict_size(50);

    let mut input = Vec::new();
    for i in 0..50 {
        input.push(i);
    }
    for i in 0..49 {
        input.push(i);
    }
    for i in 0..51 {
        input.push(i);
    }
    for i in 0..120 {
        input.push(i);
    }

    let options = PackOptions::new();
    let packed = packer.pack(&json!(input), &options).unwrap();

    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(input));

    let packed = packer.pack(&json!(input), &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(input));
}

#[test]
fn it_handles_large_messages_with_more_values_than_dictionary_size_that_contains_null() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    packer.set_max_dict_size(50);
    unpacker.set_max_dict_size(50);

    let mut input = Vec::new();
    for i in 0..50 {
        input.push(i);
    }
    for i in 0..49 {
        input.push(i);
    }
    for i in 0..51 {
        input.push(i);
    }
    for i in 0..120 {
        input.push(i);
    }

    let to_pack = json!([null, input]);

    let options = PackOptions::new();
    let packed = packer.pack(&to_pack, &options).unwrap();

    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, to_pack);

    let packed = packer.pack(&to_pack, &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, to_pack);
}

#[test]
fn it_packs_null() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer.pack(&json!(null), &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(null));
}

#[test]
fn it_packs_null_inside_object() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer.pack(&json!({ "foo": null }), &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({ "foo": null }));
}

#[test]
fn it_packs_empty_string() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer.pack(&json!(""), &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(""));
}

#[test]
fn it_packs_empty_string_inside_object() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer.pack(&json!({"foo": ""}), &options).unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({"foo": ""}));
}

#[test]
fn it_packs_multiple_empty_strings_inside_object() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer
        .pack(&json!({"foo": "", "bar": ""}), &options)
        .unwrap();
    assert_eq!(packed, json!(["foo", "bar", "", 5, 0]));
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!({"foo": "", "bar": ""}));
}

#[test]
fn it_copes_with_messages_in_wrong_order() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed1 = packer
        .pack(&json!({ "id": 1, "text": "foo" }), &options)
        .unwrap();
    let packed2 = packer
        .pack(&json!({ "id": 2, "text": "foo" }), &options)
        .unwrap();
    let packed3 = packer
        .pack(&json!({ "id": 3, "text": "foo" }), &options)
        .unwrap();

    match unpacker.unpack(&packed3) {
        Ok(s) => s,
        Err(err) => assert_eq!(
            err.cause,
            "message unpacked out of sequence or already unpacked"
        ),
    };
    match unpacker.unpack(&packed2) {
        Ok(s) => s,
        Err(err) => assert_eq!(
            err.cause,
            "message unpacked out of sequence or already unpacked"
        ),
    };
    let unpacked: Value = unpacker.unpack(&packed1).unwrap();
    assert_eq!(unpacked, json!({ "id": 1, "text": "foo" }));
}

#[test]
fn it_packs_strings_1_level_deep() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let mut options = PackOptions::new();
    options.pack_string_depth = 1;
    let packed = packer
        .pack(&json!(["foo\nbar", { "deeper": "baz\nqux" }]), &options)
        .unwrap();
    assert_eq!(
        packed,
        json!([
            TYPE_ARRAY,
            [TYPE_STRING, "foo", "bar"],
            ["deeper", "baz\nqux"],
            0
        ])
    );
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(["foo\nbar", { "deeper": "baz\nqux" }]));
}

#[test]
fn it_packs_strings_2_level_deep() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let mut options = PackOptions::new();
    options.pack_string_depth = 2;
    let packed = packer
        .pack(&json!(["foo\nbar", { "deeper": "baz\nqux" }]), &options)
        .unwrap();
    assert_eq!(
        packed,
        json!([
            TYPE_ARRAY,
            [TYPE_STRING, "foo", "bar"],
            ["deeper", [TYPE_STRING, "baz", "qux"]],
            0
        ])
    );
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!(["foo\nbar", { "deeper": "baz\nqux" }]));
}

#[test]
fn it_copes_with_null_input_to_unpacker() {
    let mut unpacker = Unpacker::new();
    assert_eq!(unpacker.unpack::<Value>(&Value::Null).unwrap(), Value::Null);
}

#[test]
fn it_packs_with_null_and_true_and_false() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer
        .pack(
            &json!({ "null": null, "true": true, "false": false }),
            &options,
        )
        .unwrap();
    assert_eq!(
        packed,
        json!(["null", "true", "false", null, true, false, 0])
    );
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(
        unpacked,
        json!({ "null": null, "true": true, "false": false })
    );

    let packed = packer
        .pack(&json!([3, 4, 5, 6, 7, 8, 1]), &options)
        .unwrap();
    let unpacked: Value = unpacker.unpack(&packed).unwrap();
    assert_eq!(unpacked, json!([3, 4, 5, 6, 7, 8, 1]));
}

#[test]
fn it_supports_reset() {
    let mut packer = Packer::new();
    let mut unpacker = Unpacker::new();

    let options = PackOptions::new();
    let packed = packer.pack(&json!({ "foo": "bar" }), &options).unwrap();
    let _unpacked: Value = unpacker.unpack(&packed).unwrap();

    let memoized = packer.pack(&json!({ "foo": "bar" }), &options).unwrap();
    assert_eq!(memoized, json!([3, 4, 1]));

    packer.reset();
    let not_memoized = packer.pack(&json!({ "foo": "bar" }), &options).unwrap();
    assert_eq!(not_memoized, json!(["foo", "bar", 0]));

    let unpacked: Value = unpacker.unpack(&not_memoized).unwrap();
    assert_eq!(unpacked, json!({ "foo": "bar" }));

    let mut unpacker = Unpacker::new();
    let unpacked: Value = unpacker.unpack(&not_memoized).unwrap();
    assert_eq!(unpacked, json!({ "foo": "bar" }));
}
