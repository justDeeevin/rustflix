use crate::args::entity_types::video_subcommands::{find_video, FindError, Video, VideoQuery};
use clap::Args;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Args)]
pub struct AddViews {
    /// The name of the video to add views to
    #[arg(long, default_value = None)]
    pub name: Option<String>,
    /// The ID of the video to add views to
    #[arg(long, default_value = None)]
    pub id: Option<u32>,
    /// The number of views to add
    #[arg(default_value_t = 1)]
    pub number_to_add: u32,
}

pub fn handle_add_views(add_views: AddViews) {
    let path = Path::new("videos.bc");
    let mut videos: Vec<Video> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    if add_views.name.is_none() && add_views.id.is_none() {
        eprintln!("You must specify either a name or an ID");
        return;
    }

    let video_query = VideoQuery {
        name: add_views.name.clone(),
        id: add_views.id.clone(),
    };

    let video = find_video(&videos, &video_query);

    if let Err(e) = video {
        match e {
            FindError::NoVideoFound => eprintln!("Update failed. No video found from given query."),
            FindError::MultipleVideosFound(counts) => {
                eprintln!("Update failed. Multiple videos found from given query.");
                if add_views.id.is_some() {
                    eprintln!("ID matches: {}", counts.id);
                }
                if add_views.name.is_some() {
                    eprintln!("Name matches: {}", counts.name);
                }
            }
        }
        return;
    }

    let video = video.unwrap();

    let video_index = videos.iter().position(|u| u == video);

    if video_index.is_none() {
        panic!("Video was found but its index wasn't. This should never happen.");
    }

    let video_index = video_index.unwrap();

    println!(
        "Successfully added {} views to {}",
        add_views.number_to_add,
        video.clone().name
    );

    let current_views = videos[video_index].views.clone();
    videos[video_index].views = current_views + add_views.number_to_add;

    let file = File::create(path).unwrap();
    bincode::serialize_into(file, &videos).unwrap();
}

pub fn handle_show_views(video_query: VideoQuery) {
    let path = Path::new("videos.bc");
    let videos: Vec<Video> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    if video_query.name.is_none() && video_query.id.is_none() {
        eprintln!("You must specify either a name or an ID");
        return;
    }

    let video = find_video(&videos, &video_query);

    if let Err(e) = video {
        match e {
            FindError::NoVideoFound => {
                eprintln!("No video found with the specified name or ID");
                return;
            }
            FindError::MultipleVideosFound(matches) => {
                eprintln!("Multiple videos found with the specified name or ID");
                if video_query.id.is_some() {
                    eprintln!("ID matches: {}", matches.id);
                }
                if video_query.name.is_some() {
                    eprintln!("Name matches: {}", matches.name);
                }
                return;
            }
        }
    }

    let video = video.unwrap();

    println!("{} has {} views", video.name, video.views);
}
