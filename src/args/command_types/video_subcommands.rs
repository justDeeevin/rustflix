use crate::utilities;
use clap::Args;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Video {
    pub id: u32,
    pub name: String,
    pub views: u32,
}

#[derive(Debug, Args)]
pub struct VideoQuery {
    /// The ID of the video to query
    #[arg(long, default_value = None)]
    pub id: Option<u32>,
    /// The name of the video to query
    #[arg(long, default_value = None)]
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct CreateVideo {
    /// The name of the video
    pub name: String,
}

/// Determines if the list of videos contains a video with the given ID
///
/// # Arguments
///
/// * `videos` - The list of videos to search
/// * `id` - The ID to search for
///
/// # Returns
///
/// * `true` if a video with the given ID is found
/// * `false` if a video with the given ID is not found
fn has_id(videos: &Vec<Video>, id: u32) -> bool {
    for video in videos {
        if video.id == id {
            return true;
        }
    }
    false
}

/// Generates an unused ID for a new video
///
/// # Arguments
///
/// * `videos` - The list of videos to check for ID conflicts
///
/// # Returns
/// A valid ID that is not already in use by a video
fn generate_valid_id(videos: &Vec<Video>) -> u32 {
    let mut rng = rand::thread_rng();
    let mut id = rng.gen_range(0..=std::u32::MAX);
    while has_id(videos, id) {
        id = rng.gen_range(0..=std::u32::MAX);
    }
    id
}

/// Handles the creation of a new video
///
/// # Arguments
///
/// * `create_video` - The arguments for the video creation
pub fn handle_create_video(create_video: CreateVideo) {
    let path = concat!(env!("HOME"), "/.rustflix/videos.bc");
    let path = Path::new(path);
    let mut videos: Vec<Video> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    let video = Video {
        id: generate_valid_id(&videos),
        name: create_video.name,
        views: 0,
    };

    videos.push(video.clone());

    let file = File::create(path).unwrap();

    bincode::serialize_into(file, &videos).unwrap();

    println!("Video created successfully");
    println!("ID: {}", video.id);
}

#[derive(Debug, Args)]
pub struct UpdateVideo {
    /// The ID of the video to update
    #[arg(long, default_value = None)]
    pub query_id: Option<u32>,
    /// The name of the video to update
    #[arg(long, default_value = None)]
    pub query_name: Option<String>,

    /// The new name of the video
    #[arg(long, default_value = None)]
    pub new_name: Option<String>,

    /// The new number of views of the video
    #[arg(long, default_value = None)]
    pub new_views: Option<u32>,
}

/// Error returned from `find_video`
///
/// # Variants
///
/// * `NoVideoFound` - No video was found matching the given query
/// * `MultipleVideosFound` - Multiple videos were found matching the given query. `RepeatedQueries` contains the number of matches for each query field.
#[derive(Debug)]
pub enum FindError {
    NoVideoFound,
    MultipleVideosFound(MatchedQueries),
}

/// Contains the number of matches for each query field
///
/// # Fields
///
/// * `id` - The number of matches for the ID query
/// * `name` - The number of matches for the name query
#[derive(Debug)]
pub struct MatchedQueries {
    pub id: u32,
    pub name: u32,
}

/// Finds a video in the given list of videos matching the given query
///
/// # Arguments
///
/// * `videos` - The list of videos to search
/// * `query` - The query to search for
///
/// # Returns
///
/// The video matching the given query. If multiple or none are found, returns a `FindError` variant matching the error case.
pub fn find_video<'a>(videos: &'a Vec<Video>, query: &VideoQuery) -> Result<&'a Video, FindError> {
    let mut found_videos: Vec<&Video> = vec![];
    let mut id_matches = 0;
    let mut name_matches = 0;

    for video in videos {
        if let Some(id) = query.id {
            if video.id == id {
                found_videos.push(video);
                id_matches += 1;
                continue;
            }
        }

        if let Some(name) = &query.name {
            if video.name == *name {
                found_videos.push(video);
                name_matches += 1;
                continue;
            }
        }
    }

    if found_videos.len() == 0 {
        return Err(FindError::NoVideoFound);
    }

    if found_videos.len() > 1 {
        return Err(FindError::MultipleVideosFound(MatchedQueries {
            id: id_matches,
            name: name_matches,
        }));
    }

    Ok(found_videos[0])
}

/// Handles the updating of an existing video
///
/// # Arguments
///
/// * `update_video` - The arguments for the video update
pub fn handle_update_video(update_video: UpdateVideo) {
    if update_video.query_id.is_none() && update_video.query_name.is_none() {
        eprintln!("No query given. Please provide an ID or name");
        return;
    }

    let path = concat!(env!("HOME"), "/.rustflix/videos.bc");
    let path = Path::new(path);
    let mut videos: Vec<Video> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    let video_query = VideoQuery {
        id: update_video.query_id.clone(),
        name: update_video.query_name.clone(),
    };

    let video = find_video(&videos, &video_query);

    if let Err(e) = video {
        match e {
            FindError::NoVideoFound => eprintln!("Update failed. No video found from given query."),
            FindError::MultipleVideosFound(counts) => {
                eprintln!("Update failed. Multiple videos found from given query.");
                if update_video.query_id.is_some() {
                    eprintln!("ID matches: {}", counts.id);
                }
                if update_video.query_name.is_some() {
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

    let og_video_state = videos[video_index].clone();

    match update_video.new_name {
        Some(ref name) => videos[video_index].name = name.clone(),
        None => {}
    }

    match update_video.new_views {
        Some(views) => {
            if !utilities::confirm(
                format!(
                    "Are you sure you want to set the views of {} to {}?",
                    videos[video_index].name, views
                )
                .as_str(),
                None,
                Some("Video update aborted."),
                Some(true),
            ) {
                return;
            }
        }
        None => {}
    }

    let file = File::create(path).unwrap();
    bincode::serialize_into(file, &videos).unwrap();

    println!("Video updated successfully.");
    if update_video.new_name.is_some() {
        println!(
            "Name changed from {} to {}",
            og_video_state.name, videos[video_index].name
        );
    }
}

pub fn handle_delete_video(video_query: VideoQuery) {
    if video_query.id.is_none() && video_query.name.is_none() {
        eprintln!("No query given. Please provide an ID or name");
        return;
    }

    let path = concat!(env!("HOME"), "/.rustflix/videos.bc");
    let path = Path::new(path);
    let mut videos: Vec<Video> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    let video = find_video(&videos, &video_query);

    if let Err(e) = video {
        match e {
            FindError::NoVideoFound => eprintln!("Delete failed. No video found from given query."),
            FindError::MultipleVideosFound(counts) => {
                eprintln!("Delete failed. Multiple videos found from given query.");
                if video_query.id.is_some() {
                    eprintln!("ID matches: {}", counts.id);
                }
                if video_query.name.is_some() {
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

    if !utilities::confirm(
        "Are you sure you want to delete this video?",
        Some(format!("{:?}", video).as_str()),
        Some("Video deletion cancelled."),
        Some(true),
    ) {
        return;
    }

    let video_index = video_index.unwrap();

    videos.remove(video_index);

    let file = File::create(path).unwrap();
    bincode::serialize_into(file, &videos).unwrap();

    println!("Video deleted successfully.");
}

fn find_videos(videos: &Vec<Video>, video_query: &VideoQuery) -> Result<Vec<Video>, FindError> {
    let mut found_videos: Vec<Video> = vec![];

    for video in videos {
        if video_query.id.is_some() {
            if video.id == video_query.id.clone().unwrap() {
                found_videos.push(video.clone());
                continue;
            }
        }

        if video_query.name.is_some() {
            if video.name == video_query.name.clone().unwrap() {
                found_videos.push(video.clone());
                continue;
            }
        }
    }

    if found_videos.len() == 0 {
        return Err(FindError::NoVideoFound);
    }

    Ok(found_videos)
}

#[derive(Debug, Args)]
pub struct ListVideo {
    /// Show all videos
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "id",
        conflicts_with = "name"
    )]
    pub all: bool,
    /// The ID of the video to query
    #[arg(long, default_value = None)]
    pub id: Option<u32>,
    /// The name of the video to query
    #[arg(long, default_value = None)]
    pub name: Option<String>,
}

pub fn handle_list_videos(show_video: ListVideo) {
    let path = concat!(env!("HOME"), "/.rustflix/videos.bc");
    let path = Path::new(path);
    let videos: Vec<Video> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    if show_video.all {
        for video in videos {
            println!("{:?}", video);
        }
        return;
    }

    if show_video.id.is_none() && show_video.name.is_none() {
        eprintln!("No query given. Please provide an ID or name");
        return;
    }

    let video_query = VideoQuery {
        id: show_video.id,
        name: show_video.name,
    };

    let found_videos = find_videos(&videos, &video_query);

    if let Err(FindError::NoVideoFound) = found_videos {
        eprintln!("No video found from given query.");
        return;
    }

    let found_videos = found_videos.unwrap();

    for video in found_videos {
        println!("{:?}", video);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_videos() -> Vec<Video> {
        vec![
            Video {
                id: 2829304751,
                name: "test".to_string(),
                views: 0,
            },
            Video {
                id: 1525162981,
                name: "test2".to_string(),
                views: 0,
            },
            Video {
                id: 986712257,
                name: "test3".to_string(),
                views: 0,
            },
            Video {
                id: 2453202404,
                name: "test4".to_string(),
                views: 0,
            },
            Video {
                id: 4036985520,
                name: "test5".to_string(),
                views: 0,
            },
        ]
    }

    #[test]
    fn test_has_id() {
        let videos = make_videos();
        assert_eq!(has_id(&videos, 2829304751), true);
        assert_eq!(has_id(&videos, 1), false);
    }

    #[test]
    fn test_generate_valid_id() {
        let videos = make_videos();
        for _ in 0..100 {
            let id = generate_valid_id(&videos);
            assert_eq!(has_id(&videos, id), false);
        }
    }
}
