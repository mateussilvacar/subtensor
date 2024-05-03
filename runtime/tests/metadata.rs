use codec::Decode;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use node_subtensor_runtime::{
    AccountId, BalancesCall, BuildStorage, Proxy, ProxyType, Runtime, RuntimeCall, RuntimeEvent,
    RuntimeGenesisConfig, RuntimeOrigin, SubtensorModule, System, SystemCall,
};
use scale_info::TypeDef;

fn is_pallet_error(segments: &Vec<String>) -> bool {
    let pallet_list: Vec<&str> = vec![
        "pallet_admin_utils",
        "pallet_collective",
        "pallet_commitments",
        "pallet_registry",
        "pallet_subtensor",
    ];
    // segments for error like pallet name, pallet, Error
    if segments.len() != 3 {
        false
    } else {
        pallet_list.contains(&segments[0].as_str())
            && segments[1] == "pallet"
            && segments[2] == "Error"
    }
}

// test make sure all errors are documented
#[test]
fn test_metadata() {
    let metadata = Runtime::metadata().1;
    // current metadata version should be 14
    assert!(matches!(metadata, RuntimeMetadata::V14(_)));

    match metadata {
        RuntimeMetadata::V14(value) => {
            let types = value.types.types;
            for ty in types.iter() {
                let segments = &ty.ty.path.segments;
                if is_pallet_error(segments) {
                    // error should be enum type
                    assert!(matches!(ty.ty.type_def, TypeDef::Variant(_)));
                    match &ty.ty.type_def {
                        TypeDef::Variant(variants) => {
                            // check docs not empty
                            for variant in variants.variants.iter() {
                                println!("{}", variant.name);
                                assert_eq!(variant.docs.len(), 1);
                                assert_eq!(variant.docs[0].is_empty(), false);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    };
}
