pub mod user_subcommands;

use clap::{Args, Subcommand};
use user_subcommands::*;

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub subcommand: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),
    /// Update an existing user by either ID, name, or email.
    Update(UpdateUser),
    /// Delete an existing user by
    Delete(UserQuery),
    /// Show a specific user
    Show(ShowUser),
}
