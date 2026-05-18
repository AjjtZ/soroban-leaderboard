#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, LeaderboardContractClient<'static>, Address) {
    let env     = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, LeaderboardContract);
    let client      = LeaderboardContractClient::new(&env, &contract_id);
    let admin       = Address::generate(&env);
    client.initialize(&admin);
    (env, client, admin)
}

// ── initialize ────────────────────────────────────────────────────────────────

#[test]
fn test_initialize_sets_season_one() {
    let (_, client, _) = setup();
    assert_eq!(client.get_season(), 1);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_initialize_twice_panics() {
    let (env, client, _) = setup();
    let other = Address::generate(&env);
    client.initialize(&other); // should panic
}

// ── submit_score ──────────────────────────────────────────────────────────────

#[test]
fn test_submit_new_player() {
    let (env, client, _) = setup();
    let player = Address::generate(&env);
    client.submit_score(&player, &String::from_str(&env, "Alice"), &1000);

    let board = client.get_leaderboard();
    assert_eq!(board.len(), 1);
    assert_eq!(board.get(0).unwrap().score, 1000);
}

#[test]
fn test_submit_higher_score_updates() {
    let (env, client, _) = setup();
    let player = Address::generate(&env);
    client.submit_score(&player, &String::from_str(&env, "Bob"), &500);
    client.submit_score(&player, &String::from_str(&env, "Bob"), &900);

    let board = client.get_leaderboard();
    assert_eq!(board.len(), 1);
    assert_eq!(board.get(0).unwrap().score, 900);
}

#[test]
fn test_submit_lower_score_ignored() {
    let (env, client, _) = setup();
    let player = Address::generate(&env);
    client.submit_score(&player, &String::from_str(&env, "Carol"), &800);
    client.submit_score(&player, &String::from_str(&env, "Carol"), &200); // lower — ignored

    let board = client.get_leaderboard();
    assert_eq!(board.get(0).unwrap().score, 800);
}

// ── get_top ───────────────────────────────────────────────────────────────────

#[test]
fn test_get_top_returns_sorted() {
    let (env, client, _) = setup();

    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    let p3 = Address::generate(&env);

    client.submit_score(&p1, &String::from_str(&env, "Alice"), &300);
    client.submit_score(&p2, &String::from_str(&env, "Bob"),   &900);
    client.submit_score(&p3, &String::from_str(&env, "Carol"), &600);

    let top = client.get_top(&2);
    assert_eq!(top.len(), 2);
    assert_eq!(top.get(0).unwrap().score, 900); // Bob first
    assert_eq!(top.get(1).unwrap().score, 600); // Carol second
}

#[test]
fn test_get_top_capped_at_list_size() {
    let (env, client, _) = setup();
    let player = Address::generate(&env);
    client.submit_score(&player, &String::from_str(&env, "Solo"), &100);

    // Requesting top 10 but only 1 player exists.
    let top = client.get_top(&10);
    assert_eq!(top.len(), 1);
}

// ── reset_season ──────────────────────────────────────────────────────────────

#[test]
fn test_reset_clears_board_and_bumps_season() {
    let (env, client, admin) = setup();
    let player = Address::generate(&env);
    client.submit_score(&player, &String::from_str(&env, "Dave"), &500);

    client.reset_season(&admin);

    assert_eq!(client.get_leaderboard().len(), 0);
    assert_eq!(client.get_season(), 2);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_reset_by_non_admin_panics() {
    let (env, client, _) = setup();
    let imposter = Address::generate(&env);
    client.reset_season(&imposter);
}