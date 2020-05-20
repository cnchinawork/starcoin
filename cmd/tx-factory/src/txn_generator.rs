// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use starcoin_executor::executor::Executor;
use starcoin_executor::TransactionExecutor;
use starcoin_types::account_address::AccountAddress;
use starcoin_types::transaction::RawUserTransaction;
use starcoin_wallet_api::WalletAccount;

pub struct MockTxnGenerator {
    receiver_address: AccountAddress,
    receiver_auth_key_prefix: Vec<u8>,
    account: WalletAccount,
}

impl MockTxnGenerator {
    pub fn new(
        account: WalletAccount,
        receiver_address: AccountAddress,
        receiver_auth_key_prefix: Vec<u8>,
    ) -> Self {
        MockTxnGenerator {
            receiver_address,
            receiver_auth_key_prefix,
            account,
        }
    }

    pub fn generate_mock_txn(&self, sequence_number: u64) -> Result<RawUserTransaction> {
        let amount_to_transfer = 1000;

        let transfer_txn = Executor::build_transfer_txn(
            self.account.address,
            self.receiver_address,
            self.receiver_auth_key_prefix.clone(),
            sequence_number,
            amount_to_transfer,
            1,
            50_000_000,
        );
        Ok(transfer_txn)
    }
}
