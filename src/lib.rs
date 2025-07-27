use async_graphql::Schema;
use firebase_auth::FirebaseAuthState;
use graphql::{MutationRoot, QueryRoot, SubscriptionRoot};

pub const DEFAULT_CACHE_EXPIRATION: u64 = 60 * 60;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub mod dataloaders;
pub mod graphql;
pub mod postgres;
pub mod redis;
pub mod simdjson;
pub mod types;

#[derive(Clone)]
pub struct AppState {
    pub auth: FirebaseAuthState,
    pub schema: Schema<QueryRoot, MutationRoot, SubscriptionRoot>,
}
