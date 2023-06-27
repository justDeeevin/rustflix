pub mod entity_types;

use clap::{Parser, Subcommand};
use entity_types::*;

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete, or show users
    User(UserCommand),
    // Create, update, delete, or show videos
    // Video(VideoCommand),
    // Add or show views on a video
    // View(ViewCommand),
}

pub fn handle_user_command(command: UserCommand) {
    match command.subcommand {
        UserSubcommand::Create(create_user) => user_subcommands::handle_create_user(create_user),
        UserSubcommand::Update(update_user) => user_subcommands::handle_update_user(update_user),
        UserSubcommand::Delete(user_query) => user_subcommands::handle_delete_user(user_query),
        UserSubcommand::Show(show_user) => user_subcommands::handle_show_user(show_user),
    }
}
