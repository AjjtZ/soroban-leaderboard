#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    vec, Env, Address, String, Vec,
};

// ─── Data Types ───────────────────────────────────────────────────────────────

/// One entry on the leaderboard.
#[contracttype]
#[derive(Clone)]
pub struct Player {
    pub address: Address,
    pub username: String,
    pub score: u64,
}

/// Storage keys
const LEADERBOARD: &str = "board";
const SEASON:      &str = "season";
const ADMIN:       &str = "admin";

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct LeaderboardContract;

#[contractimpl]
impl LeaderboardContract {

    // ── Setup ─────────────────────────────────────────────────────────────────

    /// Must be called once after deployment to register the admin wallet.
    pub fn initialize(env: Env, admin: Address) {
        let admin_key = soroban_sdk::Symbol::new(&env, ADMIN);
        if env.storage().instance().has(&admin_key) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&admin_key, &admin);
        // Start at season 1.
        env.storage()
            .instance()
            .set(&soroban_sdk::Symbol::new(&env, SEASON), &1u32);
    }

    // ── Read ──────────────────────────────────────────────────────────────────

    /// Return every player on the current leaderboard (unsorted).
    pub fn get_leaderboard(env: Env) -> Vec<Player> {
        env.storage()
            .instance()
            .get(&soroban_sdk::Symbol::new(&env, LEADERBOARD))
            .unwrap_or(vec![&env])
    }

    /// Return the current season number.
    pub fn get_season(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&soroban_sdk::Symbol::new(&env, SEASON))
            .unwrap_or(1u32)
    }

    /// Return the top N players sorted by score descending.
    pub fn get_top(env: Env, n: u32) -> Vec<Player> {
        let mut players = Self::get_leaderboard(env.clone());

        // Bubble-sort descending (small N, fine on-chain).
        let len = players.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                let a = players.get(j).unwrap();
                let b = players.get(j + 1).unwrap();
                if a.score < b.score {
                    players.set(j,     b.clone());
                    players.set(j + 1, a.clone());
                }
            }
        }

        // Take the first N entries.
        let take = (n as u32).min(players.len());
        let mut top = vec![&env];
        for i in 0..take {
            top.push_back(players.get(i).unwrap());
        }
        top
    }

    // ── Write ─────────────────────────────────────────────────────────────────

    /// Submit (or update) a score for the calling wallet.
    /// Only the highest score per address is kept.
    pub fn submit_score(env: Env, player: Address, username: String, score: u64) {
        // The player must sign this call.
        player.require_auth();

        let board_key = soroban_sdk::Symbol::new(&env, LEADERBOARD);
        let mut players: Vec<Player> = env
            .storage()
            .instance()
            .get(&board_key)
            .unwrap_or(vec![&env]);

        let mut found = false;
        for i in 0..players.len() {
            let mut p = players.get(i).unwrap();
            if p.address == player {
                // Only update if new score beats the old one.
                if score > p.score {
                    p.score    = score;
                    p.username = username.clone();
                    players.set(i, p);
                }
                found = true;
                break;
            }
        }

        if !found {
            players.push_back(Player { address: player, username, score });
        }

        env.storage().instance().set(&board_key, &players);
    }

    /// Admin-only: wipe the leaderboard and advance the season counter.
    pub fn reset_season(env: Env, admin: Address) {
        admin.require_auth();

        // Verify the caller is the registered admin.
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&soroban_sdk::Symbol::new(&env, ADMIN))
            .expect("Contract not initialized");

        if stored_admin != admin {
            panic!("Unauthorized: only admin can reset the season");
        }

        // Clear all scores.
        env.storage()
            .instance()
            .set(
                &soroban_sdk::Symbol::new(&env, LEADERBOARD),
                &vec![&env] as &Vec<Player>,
            );

        // Increment season.
        let season: u32 = Self::get_season(env.clone());
        env.storage()
            .instance()
            .set(&soroban_sdk::Symbol::new(&env, SEASON), &(season + 1));
    }
}