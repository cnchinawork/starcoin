// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

extern crate chrono;

use chrono::prelude::*;
use crypto::HashValue;

use crate::cache_storage::CacheStorage;
use crate::db_storage::DBStorage;
use crate::storage::StorageInstance;
use crate::Storage;
use starcoin_config::RocksdbConfig;
use starcoin_types::account_address::AccountAddress;
use starcoin_types::block::{Block, BlockBody, BlockHeader, BlockHeaderExtra};
use starcoin_types::genesis_config::ChainId;
use starcoin_types::transaction::SignedUserTransaction;
use starcoin_uint::U256;

#[test]
fn test_block() {
    let tmpdir = starcoin_config::temp_path();
    let storage = Storage::new(StorageInstance::new_cache_and_db_instance(
        CacheStorage::new(),
        DBStorage::new(tmpdir.path(), RocksdbConfig::default()).unwrap(),
    ))
    .unwrap();
    let dt = Local::now();

    let block_header1 = BlockHeader::new(
        HashValue::random(),
        dt.timestamp_nanos() as u64,
        1,
        AccountAddress::random(),
        HashValue::zero(),
        HashValue::random(),
        HashValue::zero(),
        0,
        U256::zero(),
        HashValue::random(),
        ChainId::test(),
        0,
        BlockHeaderExtra::new([0u8; 4]),
    );
    storage
        .block_storage
        .save_header(block_header1.clone())
        .unwrap();
    let block_id = block_header1.id();
    assert_eq!(
        block_header1,
        storage
            .block_storage
            .get_block_header_by_hash(block_id)
            .unwrap()
            .unwrap()
    );
    let block_body1 = BlockBody::new(vec![SignedUserTransaction::mock()], None);
    storage
        .block_storage
        .save_body(block_id, block_body1.clone())
        .unwrap();
    let block1 = Block::new(block_header1.clone(), block_body1);
    // save block1
    storage.block_storage.save(block1.clone()).unwrap();
    //read to block2
    let block2 = storage.block_storage.get(block_id).unwrap();
    assert!(block2.is_some());
    assert_eq!(block1, block2.unwrap());
    //get header to block3
    let block_header3 = storage
        .block_storage
        .get_block_header_by_hash(block_id)
        .unwrap()
        .unwrap();
    assert_eq!(block_header1, block_header3);
}

#[test]
fn test_block_number() {
    let tmpdir = starcoin_config::temp_path();
    let storage = Storage::new(StorageInstance::new_cache_and_db_instance(
        CacheStorage::new(),
        DBStorage::new(tmpdir.path(), RocksdbConfig::default()).unwrap(),
    ))
    .unwrap();
    let dt = Local::now();

    let block_header1 = BlockHeader::new(
        HashValue::random(),
        dt.timestamp_nanos() as u64,
        0,
        AccountAddress::random(),
        HashValue::zero(),
        HashValue::random(),
        HashValue::zero(),
        0,
        U256::zero(),
        HashValue::random(),
        ChainId::test(),
        0,
        BlockHeaderExtra::new([0u8; 4]),
    );
    storage
        .block_storage
        .save_header(block_header1.clone())
        .unwrap();
    let block_id = block_header1.id();
    assert_eq!(
        storage
            .block_storage
            .get_block_header_by_hash(block_id)
            .unwrap()
            .unwrap(),
        block_header1
    );
    let block_body1 = BlockBody::new(vec![SignedUserTransaction::mock()], None);
    storage
        .block_storage
        .save_body(block_id, block_body1.clone())
        .unwrap();
    let block1 = Block::new(block_header1, block_body1);

    // save block1
    storage.block_storage.save(block1.clone()).unwrap();

    //read to block2
    let block2 = storage.block_storage.get(block_id).unwrap();
    assert!(block2.is_some());
    assert_eq!(block1, block2.unwrap());
}
