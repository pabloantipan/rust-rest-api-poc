// src/routes/mod.rs
pub mod users;

// Re-export commonly used items
pub use users::{create_user, get_users, get_user, update_user, delete_user, AppState};