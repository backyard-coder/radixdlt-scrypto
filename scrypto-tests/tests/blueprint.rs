#![cfg_attr(not(feature = "std"), no_std)]

use scrypto::abi::BlueprintAbi;
use scrypto::buffer::*;
use scrypto::prelude::*;
use serde::Serialize;
use serde_json::{json, to_value, Value};

blueprint! {
    struct Simple {
        state: u32,
    }

    impl Simple {
        pub fn new() -> ComponentAddress {
            Self {
                state: 0
            }
            .instantiate()
            .globalize()
        }

        pub fn get_state(&self) -> u32 {
            self.state
        }

        pub fn set_state(&mut self, new_state: u32) {
            self.state = new_state;
        }

        pub fn custom_types() -> (Decimal, PackageAddress, KeyValueStore<String, String>, Hash, Bucket, Proof, Vault) {
            todo!()
        }
    }
}

fn assert_json_eq<T: Serialize>(actual: T, expected: Value) {
    assert_eq!(to_value(&actual).unwrap(), expected);
}

#[test]
fn test_simple_abi() {
    let ptr = Simple_abi(core::ptr::null_mut::<u8>());
    let abi: BlueprintAbi = scrypto_consume(ptr, |slice| scrypto_decode(slice).unwrap());

    assert_json_eq(
        abi,
        json!({
            "structure": {
                "fields":{
                    "named":[
                        [
                            "state",
                            { "type":"U32" }
                        ]
                    ],
                    "type":"Named"
                },
                "name":"Simple",
                "type":"Struct"
            },
            "fns": [
                {
                    "ident": "new",
                    "mutability": null,
                    "input": {
                        "type": "Struct",
                        "name": "Simple_new_Input",
                        "fields": {
                            "type": "Named",
                            "named": []
                        }
                    },
                    "output": {
                        "type": "Custom",
                        "type_id": 129,
                        "generics": []
                    },
                    "export_name": "Simple_new"
                },

                {
                    "ident": "get_state",
                    "mutability": "Immutable",
                    "input": {
                        "type": "Struct",
                        "name": "Simple_get_state_Input",
                        "fields": {
                            "type": "Named",
                            "named": []
                        }
                    },
                    "output": {
                        "type": "U32"
                    },
                    "export_name": "Simple_get_state"
                },
                {
                    "ident": "set_state",
                    "mutability": "Mutable",
                    "input": {
                        "type": "Struct",
                        "name": "Simple_set_state_Input",
                        "fields": {
                            "type": "Named",
                            "named": [
                                [
                                    "arg0",
                                    {
                                        "type": "U32"
                                    }
                                ]
                            ]
                        }
                    },
                    "output": {
                        "type": "Unit"
                    },
                    "export_name": "Simple_set_state"
                },
                {
                    "ident": "custom_types",
                    "mutability": null,
                    "input": {
                        "type": "Struct",
                        "name": "Simple_custom_types_Input",
                        "fields": {
                            "type": "Named",
                            "named": []
                        }
                    },
                    "output": {
                        "type": "Tuple",
                        "elements": [
                            {
                                "type": "Custom",
                                "type_id": 161,
                                "generics": []
                            },
                            {
                                "type": "Custom",
                                "type_id": 128,
                                "generics": []
                            },
                            {
                                "type": "Custom",
                                "type_id": 131,
                                "generics": [
                                    {
                                        "type": "String"
                                    },
                                    {
                                        "type": "String"
                                    }
                                ]
                            },
                            {
                                "type": "Custom",
                                "type_id": 144,
                                "generics": []
                            },
                            {
                                "type": "Custom",
                                "type_id": 177,
                                "generics": []
                            },
                            {
                                "type": "Custom",
                                "type_id": 178,
                                "generics": []
                            },
                            {
                                "type": "Custom",
                                "type_id": 179,
                                "generics": []
                            }
                        ]
                    },
                    "export_name": "Simple_custom_types"
                }
            ]
        }),
    );
}
