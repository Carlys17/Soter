#![cfg(test)]
use aid_escrow::{AidEscrow, AidEscrowClient, Config, PackageStatus};
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Env, Map, Symbol, String, Vec,
};

const UNIT: i128 = 10_000_000;

// ============================================================================
// INVARIANT TEST SUITE - Property-Based Testing for Fund Accounting
// ============================================================================
//
// This test suite verifies critical invariants across all fund operations:
// 1. Conservation of Value: Total tokens in contract == locked + claimed + available
// 2. Non-Negative Balances: No balance can ever go negative
// 3. State Consistency: Package status transitions are valid
// 4. Accounting Integrity: All operations maintain correct totals
//
// Tests use pseudo-random sequences with fixed seeds for reproducibility.
// ============================================================================

/// Helper: Setup test environment with token and contract
fn setup_test_env() -> (Env, AidEscrowClient, TokenClient, StellarAssetClient, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);

    // Register token
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_address = token_contract.address();

    let token = TokenClient::new(&env, &token_address);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);

    let contract_id = env.register(AidEscrow, ());
    let client = AidEscrowClient::new(&env, &contract_id);
    client.init(&admin);

    (env, client, token, token_admin_client, admin, token_address)
}

/// Helper: Generate pseudo-random amounts with fixed seed
fn random_amount(seed: u64, min: i128, max: i128) -> i128 {
    // Simple LCG for reproducible "random" values
    let state = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let normalized = (state >> 33) as i128;
    min + (normalized % (max - min + 1))
}

/// Helper: Verify conservation of value invariant
fn verify_conservation(
    token: &TokenClient,
    client: &AidEscrowClient,
    token_address: &Address,
    contract_address: &Address,
    total_funded: i128,
    total_claimed: i128,
) {
    let balance = token.balance(contract_address);
    let locked = client.get_total_locked(token_address);
    let claimed = client.get_total_claimed(token_address);
    
    // Invariant: balance + claimed == total_funded
    // (locked tokens are part of balance, so balance includes locked)
    assert_eq!(
        balance + claimed,
        total_funded,
        "CONSERVATION VIOLATION: balance({}) + claimed({}) != total_funded({})",
        balance, claimed, total_funded
    );
    
    // Invariant: locked <= balance
    assert!(
        locked <= balance,
        "SOLVENCY VIOLATION: locked({}) > balance({})",
        locked, balance
    );
    
    // Invariant: claimed <= total_funded
    assert!(
        claimed <= total_funded,
        "OVERCLAIM VIOLATION: claimed({}) > total_funded({})",
        claimed, total_funded
    );
}

/// Helper: Verify non-negative balances invariant
fn verify_non_negative(
    token: &TokenClient,
    client: &AidEscrowClient,
    token_address: &Address,
    contract_address: &Address,
) {
    let balance = token.balance(contract_address);
    let locked = client.get_total_locked(token_address);
    let claimed = client.get_total_claimed(token_address);
    
    assert!(balance >= 0, "NEGATIVE BALANCE: balance = {}", balance);
    assert!(locked >= 0, "NEGATIVE LOCKED: locked = {}", locked);
    assert!(claimed >= 0, "NEGATIVE CLAIMED: claimed = {}", claimed);
}

// ============================================================================
// TEST 1: Core Accounting Invariants (Original Test - Enhanced)
// ============================================================================

#[test]
fn test_core_accounting_invariants() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();

    // 1. Funding Invariant
    let fund_amount = 50 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);

    // 2. Creation Invariant: Locked + Surplus == Balance
    client.create_package(
        &admin,
        &1,
        &admin,
        &(10 * UNIT),
        &token_address,
        &0,
        &Map::new(&env),
    );

    let locked = client.get_total_locked(&token_address);
    let balance = token.balance(&client.address);

    assert_eq!(locked, 10 * UNIT);
    assert!(balance >= locked, "Contract must be solvent");

    // 3. Claim Invariant: Total Claimed + Current Balance == Total Funded
    client.claim(&1);

    let total_claimed = client.get_total_claimed(&token_address);
    let current_balance = token.balance(&client.address);
    let final_locked = client.get_total_locked(&token_address);

    assert_eq!(final_locked, 0, "Locked should return to zero");
    assert_eq!(
        total_claimed,
        10 * UNIT,
        "Claimed map should record 10 units"
    );
    assert_eq!(
        total_claimed + current_balance,
        fund_amount,
        "Conservation of value failed"
    );
}

// ============================================================================
// TEST 2: Multi-Package Conservation Invariant
// ============================================================================

#[test]
fn test_multi_package_conservation() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let fund_amount = 100 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);
    
    // Create 5 packages with varying amounts
    let amounts = [10 * UNIT, 15 * UNIT, 20 * UNIT, 5 * UNIT, 8 * UNIT];
    let mut total_locked = 0i128;
    
    for (i, &amount) in amounts.iter().enumerate() {
        let pkg_id = (i + 1) as u64;
        client.create_package(
            &admin,
            &pkg_id,
            &admin,
            &amount,
            &token_address,
            &0,
            &Map::new(&env),
        );
        total_locked += amount;
        
        // Verify invariant after each creation
        verify_conservation(
            &token,
            &client,
            &token_address,
            &client.address,
            fund_amount,
            0,
        );
        verify_non_negative(&token, &client, &token_address, &client.address);
    }
    
    assert_eq!(client.get_total_locked(&token_address), total_locked);
    
    // Claim packages one by one
    let mut total_claimed = 0i128;
    for i in 0..amounts.len() {
        let pkg_id = (i + 1) as u64;
        client.claim(&pkg_id);
        total_claimed += amounts[i];
        
        // Verify invariant after each claim
        verify_conservation(
            &token,
            &client,
            &token_address,
            &client.address,
            fund_amount,
            total_claimed,
        );
        verify_non_negative(&token, &client, &token_address, &client.address);
    }
    
    assert_eq!(client.get_total_claimed(&token_address), total_claimed);
    assert_eq!(client.get_total_locked(&token_address), 0);
}

// ============================================================================
// TEST 3: Randomized State Transitions with Fixed Seed
// ============================================================================

#[test]
fn test_randomized_state_transitions() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let fund_amount = 200 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);
    
    let mut total_funded = fund_amount;
    let mut total_claimed = 0i128;
    let mut packages_created = Vec::new(&env);
    let mut seed = 12345u64;
    
    // Phase 1: Create random packages
    for i in 0..10 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let amount = random_amount(seed, 1 * UNIT, 15 * UNIT);
        let pkg_id = (i + 1) as u64;
        
        client.create_package(
            &admin,
            &pkg_id,
            &admin,
            &amount,
            &token_address,
            &0,
            &Map::new(&env),
        );
        packages_created.push_back(pkg_id);
        
        // Verify invariants
        verify_conservation(
            &token,
            &client,
            &token_address,
            &client.address,
            total_funded,
            total_claimed,
        );
        verify_non_negative(&token, &client, &token_address, &client.address);
    }
    
    // Phase 2: Randomly claim some packages
    for i in 0..5 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let idx = (seed as usize) % packages_created.len();
        let pkg_id = packages_created.get(idx as u32);
        
        // Get package details before claim
        let pkg = client.get_package(&pkg_id);
        if pkg.status == PackageStatus::Created {
            let amount = pkg.amount;
            client.claim(&pkg_id);
            total_claimed += amount;
            
            // Verify invariants
            verify_conservation(
                &token,
                &client,
                &token_address,
                &client.address,
                total_funded,
                total_claimed,
            );
            verify_non_negative(&token, &client, &token_address, &client.address);
        }
    }
}

// ============================================================================
// TEST 4: Revoke and Refund Invariants
// ============================================================================

#[test]
fn test_revoke_refund_invariants() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let fund_amount = 100 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);
    
    // Create packages
    client.create_package(&admin, &1, &admin, &(20 * UNIT), &token_address, &0, &Map::new(&env));
    client.create_package(&admin, &2, &admin, &(30 * UNIT), &token_address, &0, &Map::new(&env));
    client.create_package(&admin, &3, &admin, &(15 * UNIT), &token_address, &0, &Map::new(&env));
    
    let mut total_claimed = 0i128;
    
    // Claim package 1
    client.claim(&1);
    total_claimed += 20 * UNIT;
    
    verify_conservation(
        &token,
        &client,
        &token_address,
        &client.address,
        fund_amount,
        total_claimed,
    );
    
    // Revoke package 2
    client.revoke(&2);
    
    verify_conservation(
        &token,
        &client,
        &token_address,
        &client.address,
        fund_amount,
        total_claimed,
    );
    verify_non_negative(&token, &client, &token_address, &client.address);
    
    // Refund package 3
    client.refund(&3);
    
    verify_conservation(
        &token,
        &client,
        &token_address,
        &client.address,
        fund_amount,
        total_claimed,
    );
    verify_non_negative(&token, &client, &token_address, &client.address);
    
    // Final state: only package 1 claimed
    assert_eq!(client.get_total_claimed(&token_address), total_claimed);
    assert_eq!(client.get_total_locked(&token_address), 0);
}

// ============================================================================
// TEST 5: Edge Case - Zero and Minimum Amounts
// ============================================================================

#[test]
fn test_edge_case_minimum_amounts() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let fund_amount = 10 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);
    
    // Create package with minimum amount (1 stroop)
    client.create_package(&admin, &1, &admin, &1, &token_address, &0, &Map::new(&env));
    
    verify_conservation(
        &token,
        &client,
        &token_address,
        &client.address,
        fund_amount,
        0,
    );
    
    client.claim(&1);
    
    verify_conservation(
        &token,
        &client,
        &token_address,
        &client.address,
        fund_amount,
        1,
    );
    
    assert_eq!(client.get_total_claimed(&token_address), 1);
}

// ============================================================================
// TEST 6: Large Scale Stress Test
// ============================================================================

#[test]
fn test_large_scale_stress() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let fund_amount = 1000 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);
    
    let mut total_claimed = 0i128;
    let mut seed = 99999u64;
    
    // Create and claim 50 packages
    for i in 0..50 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let amount = random_amount(seed, 1 * UNIT, 10 * UNIT);
        let pkg_id = (i + 1) as u64;
        
        client.create_package(
            &admin,
            &pkg_id,
            &admin,
            &amount,
            &token_address,
            &0,
            &Map::new(&env),
        );
        
        client.claim(&pkg_id);
        total_claimed += amount;
        
        // Verify invariants every 10 iterations
        if (i + 1) % 10 == 0 {
            verify_conservation(
                &token,
                &client,
                &token_address,
                &client.address,
                fund_amount,
                total_claimed,
            );
            verify_non_negative(&token, &client, &token_address, &client.address);
        }
    }
    
    // Final verification
    verify_conservation(
        &token,
        &client,
        &token_address,
        &client.address,
        fund_amount,
        total_claimed,
    );
    
    assert_eq!(client.get_total_locked(&token_address), 0);
    assert_eq!(client.get_total_claimed(&token_address), total_claimed);
}

// ============================================================================
// TEST 7: Mixed Operations Invariant
// ============================================================================

#[test]
fn test_mixed_operations_invariant() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let fund_amount = 150 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);
    
    let mut total_claimed = 0i128;
    
    // Create 10 packages
    for i in 0..10 {
        let pkg_id = (i + 1) as u64;
        let amount = (10 + i) * UNIT;
        client.create_package(
            &admin,
            &pkg_id,
            &admin,
            &amount,
            &token_address,
            &0,
            &Map::new(&env),
        );
    }
    
    // Mixed operations: claim, revoke, refund
    client.claim(&1);
    total_claimed += 10 * UNIT;
    verify_conservation(&token, &client, &token_address, &client.address, fund_amount, total_claimed);
    
    client.revoke(&2);
    verify_conservation(&token, &client, &token_address, &client.address, fund_amount, total_claimed);
    
    client.claim(&3);
    total_claimed += 12 * UNIT;
    verify_conservation(&token, &client, &token_address, &client.address, fund_amount, total_claimed);
    
    client.refund(&4);
    verify_conservation(&token, &client, &token_address, &client.address, fund_amount, total_claimed);
    
    client.claim(&5);
    total_claimed += 14 * UNIT;
    verify_conservation(&token, &client, &token_address, &client.address, fund_amount, total_claimed);
    
    // Final state
    assert_eq!(client.get_total_locked(&token_address), 0);
    assert_eq!(client.get_total_claimed(&token_address), total_claimed);
    verify_non_negative(&token, &client, &token_address, &client.address);
}

// ============================================================================
// TEST 8: Balance Never Negative After All Operations
// ============================================================================

#[test]
fn test_balance_never_negative() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let fund_amount = 50 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);
    
    // Create and immediately claim/revoke/refund
    for i in 0..20 {
        let pkg_id = (i + 1) as u64;
        let amount = 2 * UNIT;
        
        client.create_package(
            &admin,
            &pkg_id,
            &admin,
            &amount,
            &token_address,
            &0,
            &Map::new(&env),
        );
        
        // Verify after creation
        verify_non_negative(&token, &client, &token_address, &client.address);
        
        match i % 3 {
            0 => client.claim(&pkg_id),
            1 => client.revoke(&pkg_id),
            _ => client.refund(&pkg_id),
        }
        
        // Verify after operation
        verify_non_negative(&token, &client, &token_address, &client.address);
    }
}

// ============================================================================
// TEST 9: Conservation After Multiple Funding Rounds
// ============================================================================

#[test]
fn test_conservation_multiple_funding_rounds() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let mut total_funded = 0i128;
    let mut total_claimed = 0i128;
    
    // Multiple funding rounds
    for i in 0..5 {
        let fund_amount = (20 + i * 10) * UNIT;
        token_admin_client.mint(&admin, &fund_amount);
        client.fund(&token_address, &admin, &fund_amount);
        total_funded += fund_amount;
        
        verify_conservation(
            &token,
            &client,
            &token_address,
            &client.address,
            total_funded,
            total_claimed,
        );
    }
    
    // Create and claim packages
    for i in 0..10 {
        let pkg_id = (i + 1) as u64;
        let amount = 5 * UNIT;
        
        client.create_package(
            &admin,
            &pkg_id,
            &admin,
            &amount,
            &token_address,
            &0,
            &Map::new(&env),
        );
        
        client.claim(&pkg_id);
        total_claimed += amount;
        
        verify_conservation(
            &token,
            &client,
            &token_address,
            &client.address,
            total_funded,
            total_claimed,
        );
    }
    
    assert_eq!(client.get_total_claimed(&token_address), total_claimed);
}

// ============================================================================
// TEST 10: State Transition Validity
// ============================================================================

#[test]
fn test_state_transition_validity() {
    let (env, client, token, token_admin_client, admin, token_address) = setup_test_env();
    
    let fund_amount = 100 * UNIT;
    token_admin_client.mint(&admin, &fund_amount);
    client.fund(&token_address, &admin, &fund_amount);
    
    // Test all valid state transitions
    client.create_package(&admin, &1, &admin, &(10 * UNIT), &token_address, &0, &Map::new(&env));
    let pkg = client.get_package(&1);
    assert_eq!(pkg.status, PackageStatus::Created);
    
    client.claim(&1);
    let pkg = client.get_package(&1);
    assert_eq!(pkg.status, PackageStatus::Claimed);
    
    client.create_package(&admin, &2, &admin, &(10 * UNIT), &token_address, &0, &Map::new(&env));
    client.revoke(&2);
    let pkg = client.get_package(&2);
    assert_eq!(pkg.status, PackageStatus::Expired);
    
    client.create_package(&admin, &3, &admin, &(10 * UNIT), &token_address, &0, &Map::new(&env));
    client.refund(&3);
    let pkg = client.get_package(&3);
    assert_eq!(pkg.status, PackageStatus::Refunded);
    
    client.create_package(&admin, &4, &admin, &(10 * UNIT), &token_address, &0, &Map::new(&env));
    client.cancel_package(&4);
    let pkg = client.get_package(&4);
    assert_eq!(pkg.status, PackageStatus::Cancelled);
}
