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
    /// Create, update, delete, or show videos
    Video(VideoCommand),
    /// Add or show views on a video
    View(ViewCommand),
}

pub fn handle_user_command(command: UserCommand) {
    match command.subcommand {
        UserSubcommand::Create(create_user) => user_subcommands::handle_create_user(create_user),
        UserSubcommand::Update(update_user) => user_subcommands::handle_update_user(update_user),
        UserSubcommand::Delete(user_query) => user_subcommands::handle_delete_user(user_query),
        UserSubcommand::List(show_user) => user_subcommands::handle_list_users(show_user),
    }
}

pub fn handle_video_command(command: VideoCommand) {
    match command.subcommand {
        VideoSubcommand::Create(create_video) => {
            video_subcommands::handle_create_video(create_video)
        }

        VideoSubcommand::Update(update_video) => {
            video_subcommands::handle_update_video(update_video)
        }

        VideoSubcommand::Delete(video_query) => video_subcommands::handle_delete_video(video_query),
        VideoSubcommand::List(show_video) => video_subcommands::handle_list_videos(show_video),
    }
}

pub fn handle_view_command(command: ViewCommand) {
    match command.subcommand {
        ViewSubcommand::Add(add_views) => view_subcommands::handle_add_views(add_views),
        ViewSubcommand::Show(video_query) => view_subcommands::handle_show_views(video_query),
    }
}
