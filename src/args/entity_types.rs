pub mod user_subcommands;
pub mod video_subcommands;

use clap::{Args, Subcommand};
use user_subcommands::*;
use video_subcommands::*;

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
    /// List one or more users
    List(ShowUser),
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub subcommand: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create(CreateVideo),
    /// Update an existing video by either ID or name
    Update(UpdateVideo),
    /// Delete an existing video by either ID or name
    Delete(VideoQuery),
    /// List one or more videos
    List(ListVideo),
}
