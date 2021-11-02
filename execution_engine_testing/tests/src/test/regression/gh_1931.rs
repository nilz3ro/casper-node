use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_ADDR, DEFAULT_PAYMENT, DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_types::{runtime_args, CLValue, Key, RuntimeArgs};
use core::convert::TryFrom;

const CONTRACT_SET_NAMED_KEY_STORED: &str = "set_named_key_stored.wasm";
const ARG_VALUE_TO_SET: &str = "value_to_set";
const NAMED_KEY: &str = "expected_named_key";
const VALUE_TO_SET: &str = "424242";
const HASH_KEY_NAME: &str = "set_named_key";
const PACKAGE_HASH_KEY_NAME: &str = "set_named_key_package_hash";
const STORED_CONTRACT_ENTRY_POINT_NAME: &str = "process_set_key";

#[ignore]
#[test]
fn main() {
    let deploy_item = DeployItemBuilder::default()
        .with_address(*DEFAULT_ACCOUNT_ADDR)
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_session_code(
            &CONTRACT_SET_NAMED_KEY_STORED,
            runtime_args! { ARG_VALUE_TO_SET => VALUE_TO_SET},
        )
        .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
        .build();

    let exec_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

    let mut builder = InMemoryWasmTestBuilder::default();
    builder
        .run_genesis(&DEFAULT_RUN_GENESIS_REQUEST)
        .exec(exec_request)
        .expect_success()
        .commit();

    let _stored_contract_hash = builder
        .query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[HASH_KEY_NAME.to_string()],
        )
        .unwrap();

    let deploy_item = DeployItemBuilder::default()
        .with_address(*DEFAULT_ACCOUNT_ADDR)
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_stored_versioned_contract_by_name(
            PACKAGE_HASH_KEY_NAME,
            None,
            STORED_CONTRACT_ENTRY_POINT_NAME,
            runtime_args! {ARG_VALUE_TO_SET => VALUE_TO_SET},
        )
        .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
        .build();

    let exec_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

    builder.exec(exec_request).expect_success().commit();

    let actual_value = builder
        .query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[PACKAGE_HASH_KEY_NAME.to_string(), NAMED_KEY.to_string()],
            // &[PACKAGE_HASH_KEY_NAME.to_string(), NAMED_KEY.to_string()],
        )
        .expect("failed to query named key.");

    let actual_value = CLValue::try_from(actual_value)
        .expect("failed to convert StoredValue into CLValue.")
        .into_t::<String>()
        .expect("failed to convert CLValue into String.");

    println!("actual? {:?}", actual_value);
}
