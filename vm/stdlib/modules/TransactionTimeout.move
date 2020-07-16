address 0x1 {

module TransactionTimeout {
  use 0x1::Signer;
  use 0x1::CoreAddresses;
  use 0x1::Timestamp;

  resource struct TTL {
    // Only transactions with timestamp in between block time and block time + duration would be accepted.
    duration_microseconds: u64,
  }

  public fun initialize(account: &signer) {
    // Only callable by the Genesis address
    assert(Signer::address_of(account) == CoreAddresses::GENESIS_ACCOUNT(), 1);
    // Currently set to 1day.
    //TODO set by onchain config.
    move_to(account, TTL {duration_microseconds: 86400000000});
  }
  spec fun initialize {
    aborts_if Signer::get_address(account) != CoreAddresses::GENESIS_ACCOUNT();
    aborts_if exists<TTL>(Signer::get_address(account));
    ensures global<TTL>(Signer::get_address(account)).duration_microseconds == 86400000000;
  }

  public fun set_timeout(account: &signer, new_duration: u64) acquires TTL {
    // Only callable by the Genesis address
    assert(Signer::address_of(account) == CoreAddresses::GENESIS_ACCOUNT(), 1);

    let timeout = borrow_global_mut<TTL>(CoreAddresses::GENESIS_ACCOUNT());
    timeout.duration_microseconds = new_duration;
  }
  spec fun set_timeout {
    aborts_if Signer::get_address(account) != 1;
    aborts_if !exists<TTL>(CoreAddresses::GENESIS_ACCOUNT());
    ensures global<TTL>(Signer::get_address(account)).duration_microseconds == new_duration;
  }

  public fun is_valid_transaction_timestamp(timestamp: u64): bool acquires TTL {
    // Reject timestamp greater than u64::MAX / 1_000_000;
    if(timestamp > 9223372036854) {
      return false
    };

    let current_block_time = Timestamp::now_microseconds();
    let timeout = borrow_global<TTL>(CoreAddresses::GENESIS_ACCOUNT()).duration_microseconds;
    let _max_txn_time = current_block_time + timeout;

    let txn_time_microseconds = timestamp * 1000000;
    // TODO: Add Timestamp::is_before_exclusive(&txn_time_microseconds, &max_txn_time)
    //       This is causing flaky test right now. The reason is that we will use this logic for AC, where its wall
    //       clock time might be out of sync with the real block time stored in StateStore.
    //       See details in issue #2346.
    current_block_time < txn_time_microseconds
  }
  spec fun is_valid_transaction_timestamp {
    aborts_if timestamp <= 9223372036854 && !exists<Timestamp::CurrentTimeMicroseconds>(CoreAddresses::GENESIS_ACCOUNT());
    aborts_if timestamp <= 9223372036854 && !exists<TTL>(CoreAddresses::GENESIS_ACCOUNT());
    aborts_if timestamp <= 9223372036854 && global<Timestamp::CurrentTimeMicroseconds>(CoreAddresses::GENESIS_ACCOUNT()).microseconds + global<TTL>(CoreAddresses::GENESIS_ACCOUNT()).duration_microseconds > max_u64();
    aborts_if timestamp <= 9223372036854 && timestamp * 1000000 > max_u64();
    ensures timestamp > 9223372036854 ==> result == false;
    ensures timestamp <= 9223372036854 ==> result == (global<Timestamp::CurrentTimeMicroseconds>(CoreAddresses::GENESIS_ACCOUNT()).microseconds < timestamp * 1000000);
  }
}
}
