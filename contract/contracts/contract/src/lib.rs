#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct CodeSnippet {
    pub id: u64,
    pub author: String,
    pub title: String,
    pub language: String,
    pub content: String,
}

#[contracttype]
pub enum CodeBook {
    Snippet(u64),
    Counter,
}

#[contract]
pub struct CodeSharingContract;

#[contractimpl]
impl CodeSharingContract {
    // Share a new code snippet
    pub fn share_code(
        env: Env,
        author: String,
        title: String,
        language: String,
        content: String,
    ) -> u64 {
        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&CodeBook::Counter)
            .unwrap_or(0);

        counter += 1;

        let snippet = CodeSnippet {
            id: counter,
            author,
            title,
            language,
            content,
        };

        env.storage()
            .instance()
            .set(&CodeBook::Snippet(counter), &snippet);

        env.storage().instance().set(&CodeBook::Counter, &counter);

        counter
    }

    // Get a code snippet by ID
    pub fn get_code(env: Env, id: u64) -> CodeSnippet {
        env.storage()
            .instance()
            .get(&CodeBook::Snippet(id))
            .unwrap()
    }

    // Get total number of shared snippets
    pub fn total_codes(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&CodeBook::Counter)
            .unwrap_or(0)
    }
}

