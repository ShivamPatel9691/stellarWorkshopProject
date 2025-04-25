#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, symbol_short, Symbol, log};

#[contracttype]
#[derive(Clone)]
pub struct GameStats {
    pub user_id: u64,
    pub calories_burned: u64,
    pub session_time: u64,
}

#[contracttype]
pub enum FitnessBook {
    Record(u64),
}

const USER_COUNT: Symbol = symbol_short!("U_COUNT");

#[contract]
pub struct VRFitnessContract;

#[contractimpl]
impl VRFitnessContract {
    pub fn register_user(env: Env) -> u64 {
        let mut user_count = env.storage().instance().get(&USER_COUNT).unwrap_or(0);
        user_count += 1;
        env.storage().instance().set(&USER_COUNT, &user_count);

        let stats = GameStats {
            user_id: user_count,
            calories_burned: 0,
            session_time: 0,
        };

        env.storage().instance().set(&FitnessBook::Record(user_count), &stats);
        log!(&env, "User registered with ID: {}", user_count);

        user_count
    }

    pub fn update_stats(env: Env, user_id: u64, calories: u64, time: u64) {
        let mut stats = env.storage().instance().get(&FitnessBook::Record(user_id)).unwrap_or(GameStats {
            user_id,
            calories_burned: 0,
            session_time: 0,
        });

        stats.calories_burned += calories;
        stats.session_time += time;

        env.storage().instance().set(&FitnessBook::Record(user_id), &stats);
        log!(&env, "Stats updated for User ID: {}", user_id);
    }

    pub fn get_stats(env: Env, user_id: u64) -> GameStats {
        env.storage().instance().get(&FitnessBook::Record(user_id)).unwrap_or(GameStats {
            user_id,
            calories_burned: 0,
            session_time: 0,
        })
    }
}
