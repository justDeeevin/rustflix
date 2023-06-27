mod args;
use args::{EntityType, RustflixArgs};
use clap::Parser;

fn main() {
    let args = RustflixArgs::parse();

    match args.entity_type {
        EntityType::User(user_command) => args::handle_user_command(user_command),
    }
}
