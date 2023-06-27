mod args;
use args::{CommandType, RustflixArgs};
use clap::Parser;

static OUT_DIR: &str = env!("OUT_DIR");

fn main() {
    let args = RustflixArgs::parse();

    match args.command_type {
        CommandType::User(user_command) => args::handle_user_command(user_command),
        CommandType::Video(video_command) => args::handle_video_command(video_command),
        CommandType::View(view_command) => args::handle_view_command(view_command),
    }
}
