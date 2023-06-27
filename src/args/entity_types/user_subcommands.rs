use clap::Args;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UserQuery {
    /// The ID of the user to query
    #[arg(long, default_value = None)]
    pub id: Option<u32>,
    /// The name of the user to query
    #[arg(long, default_value = None)]
    pub name: Option<String>,
    /// The email address of the user to query
    #[arg(long, default_value = None)]
    pub email: Option<String>,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,
    /// The email address of the user
    pub email: String,
}

/// Determines if the list of users contains a user with the given ID
///
/// # Arguments
///
/// * `users` - The list of users to search
/// * `id` - The ID to search for
///
/// # Returns
///
/// * `true` if a user with the given ID is found
/// * `false` if a user with the given ID is not found
fn has_id(users: &Vec<User>, id: u32) -> bool {
    for user in users {
        if user.id == id {
            return true;
        }
    }
    false
}

/// Determines if the list of users contains a user with the given email address
///
/// # Arguments
///
/// * `users` - The list of users to search
/// * `email` - The email address to search for
///
/// # Returns
///
/// * `true` if a user with the given email address is found
/// * `false` if a user with the given email address is not found
fn has_email(users: &Vec<User>, email: &String) -> bool {
    for user in users {
        if user.email == *email {
            return true;
        }
    }
    false
}

/// Generates an unused ID for a new user
///
/// # Arguments
///
/// * `users` - The list of users to check for ID conflicts
///
/// # Returns
/// A valid ID that is not already in use by a user
fn generate_valid_id(users: &Vec<User>) -> u32 {
    let mut rng = rand::thread_rng();
    let mut id = rng.gen_range(0..=std::u32::MAX);
    while has_id(users, id) {
        id = rng.gen_range(0..=std::u32::MAX);
    }
    id
}

/// Handles the creation of a new user
///
/// # Arguments
///
/// * `create_user` - The arguments for the user creation
pub fn handle_create_user(create_user: CreateUser) {
    let path = Path::new("users.bc");
    let mut users: Vec<User> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    if has_email(&users, &create_user.email) {
        eprintln!("User not generated. Given email already exists");
        return;
    }

    let user = User {
        id: generate_valid_id(&users),
        name: create_user.name,
        email: create_user.email,
    };

    users.push(user.clone());

    let file = File::create(path).unwrap();

    bincode::serialize_into(file, &users).unwrap();

    println!("User created successfully");
    println!("ID: {}", user.id);
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The ID of the user to update
    #[arg(long, default_value = None)]
    pub query_id: Option<u32>,
    /// The name of the user to update
    #[arg(long, default_value = None)]
    pub query_name: Option<String>,
    /// The email address of the user to update
    #[arg(long, default_value = None)]
    pub query_email: Option<String>,

    /// The new name of the user
    #[arg(long, default_value = None)]
    pub new_name: Option<String>,

    /// The new email address of the user
    #[arg(long, default_value = None)]
    pub new_email: Option<String>,
}

/// Error returned from `find_user`
///
/// # Variants
///
/// * `NoUserFound` - No user was found matching the given query
/// * `MultipleUsersFound` - Multiple users were found matching the given query. `RepeatedQueries` contains the number of matches for each query field.
#[derive(Debug)]
enum FindError {
    NoUserFound,
    MultipleUsersFound(MatchedQueries),
}

/// Contains the number of matches for each query field
///
/// # Fields
///
/// * `id` - The number of matches for the ID query
/// * `name` - The number of matches for the name query
/// * `email` - The number of matches for the email query
#[derive(Debug)]
struct MatchedQueries {
    id: u32,
    name: u32,
    email: u32,
}

/// Finds a user in the given list of users matching the given query
///
/// # Arguments
///
/// * `users` - The list of users to search
/// * `query` - The query to search for
///
/// # Returns
///
/// The user matching the given query. If multiple or none are found, returns a `FindError` variant matching the error case.
fn find_user<'a>(users: &'a Vec<User>, query: &UserQuery) -> Result<&'a User, FindError> {
    let mut found_users: Vec<&User> = vec![];
    let mut id_matches = 0;
    let mut name_matches = 0;
    let mut email_matches = 0;

    for user in users {
        if let Some(id) = query.id {
            if user.id == id {
                found_users.push(user);
                id_matches += 1;
                continue;
            }
        }

        if let Some(name) = &query.name {
            if user.name == *name {
                found_users.push(user);
                name_matches += 1;
                continue;
            }
        }

        if let Some(email) = &query.email {
            if user.email == *email {
                found_users.push(user);
                email_matches += 1;
                continue;
            }
        }
    }

    if found_users.len() == 0 {
        return Err(FindError::NoUserFound);
    }

    if found_users.len() > 1 {
        return Err(FindError::MultipleUsersFound(MatchedQueries {
            id: id_matches,
            name: name_matches,
            email: email_matches,
        }));
    }

    Ok(found_users[0])
}

/// Handles the updating of an existing user
///
/// # Arguments
///
/// * `update_user` - The arguments for the user update
pub fn handle_update_user(update_user: UpdateUser) {
    if update_user.query_id.is_none()
        && update_user.query_name.is_none()
        && update_user.query_email.is_none()
    {
        eprintln!("No query given. Please provide an ID, name, or email");
        return;
    }

    let path = Path::new("users.bc");
    let mut users: Vec<User> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    let user_query = UserQuery {
        id: update_user.query_id.clone(),
        name: update_user.query_name.clone(),
        email: update_user.query_email.clone(),
    };

    let user = find_user(&users, &user_query);

    if let Err(e) = user {
        match e {
            FindError::NoUserFound => eprintln!("Update failed. No user found from given query."),
            FindError::MultipleUsersFound(counts) => {
                eprintln!("Update failed. Multiple users found from given query.");
                if update_user.query_id.is_some() {
                    eprintln!("ID matches: {}", counts.id);
                }
                if update_user.query_name.is_some() {
                    eprintln!("Name matches: {}", counts.name);
                }
                if update_user.query_email.is_some() {
                    eprintln!("Email matches: {}", counts.email);
                }
            }
        }
        return;
    }

    let user = user.unwrap();

    let user_index = users.iter().position(|u| u == user);

    if user_index == None {
        panic!("User was found but its index wasn't. This should never happen.");
    }

    let user_index = user_index.unwrap();

    let og_user_state = users[user_index].clone();

    match update_user.new_name {
        Some(ref name) => users[user_index].name = name.clone(),
        None => {}
    }

    match update_user.new_email {
        Some(ref email) => users[user_index].email = email.clone(),
        None => {}
    }

    let file = File::create(path).unwrap();
    bincode::serialize_into(file, &users).unwrap();

    println!("User updated successfully.");
    if update_user.new_email.is_some() {
        println!(
            "Email changed from {} to {}",
            og_user_state.email, users[user_index].email
        );
    }
    if update_user.new_name.is_some() {
        println!(
            "Name changed from {} to {}",
            og_user_state.name, users[user_index].name
        );
    }
}

pub fn handle_delete_user(user_query: UserQuery) {
    if user_query.id.is_none() && user_query.name.is_none() && user_query.email.is_none() {
        eprintln!("No query given. Please provide an ID, name, or email");
        return;
    }

    let path = Path::new("users.bc");
    let mut users: Vec<User> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    let user = find_user(&users, &user_query);

    if let Err(e) = user {
        match e {
            FindError::NoUserFound => eprintln!("Delete failed. No user found from given query."),
            FindError::MultipleUsersFound(counts) => {
                eprintln!("Delete failed. Multiple users found from given query.");
                if user_query.id.is_some() {
                    eprintln!("ID matches: {}", counts.id);
                }
                if user_query.name.is_some() {
                    eprintln!("Name matches: {}", counts.name);
                }
                if user_query.email.is_some() {
                    eprintln!("Email matches: {}", counts.email);
                }
            }
        }
        return;
    }

    let user = user.unwrap();

    let user_index = users.iter().position(|u| u == user);

    if user_index == None {
        panic!("User was found but its index wasn't. This should never happen.");
    }

    println!(
        "Are you sure you want to remove this user? ([Y]es/[n]o)\n{:?}",
        user
    );

    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_lowercase();
        if input == "n" || input == "no" {
            println!("User deletion cancelled.");
            return;
        } else if input == "" {
        } else if input != "y" && input != "yes" {
            eprintln!("Invalid input");
            input = "".to_string();
            continue;
        }
        break;
    }

    let user_index = user_index.unwrap();

    users.remove(user_index);

    let file = File::create(path).unwrap();
    bincode::serialize_into(file, &users).unwrap();

    println!("User deleted successfully.");
}

fn find_users(users: &Vec<User>, user_query: &UserQuery) -> Result<Vec<User>, FindError> {
    let mut found_users: Vec<User> = vec![];

    for user in users {
        if user_query.id.is_some() {
            if user.id == user_query.id.clone().unwrap() {
                found_users.push(user.clone());
                continue;
            }
        }

        if user_query.name.is_some() {
            if user.name == user_query.name.clone().unwrap() {
                found_users.push(user.clone());
                continue;
            }
        }

        if user_query.email.is_some() {
            if user.email == user_query.email.clone().unwrap() {
                found_users.push(user.clone());
                continue;
            }
        }
    }

    if found_users.len() == 0 {
        return Err(FindError::NoUserFound);
    }

    Ok(found_users)
}

#[derive(Debug, Args)]
pub struct ShowUser {
    /// Show all users
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "id",
        conflicts_with = "name",
        conflicts_with = "email"
    )]
    pub all: bool,
    /// The ID of the user to query
    #[arg(long, default_value = None)]
    pub id: Option<u32>,
    /// The name of the user to query
    #[arg(long, default_value = None)]
    pub name: Option<String>,
    /// The email address of the user to query
    #[arg(long, default_value = None)]
    pub email: Option<String>,
}

pub fn handle_list_users(show_user: ShowUser) {
    let path = Path::new("users.bc");
    let users: Vec<User> = if path.exists() {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    } else {
        vec![]
    };

    if show_user.all {
        for user in users {
            println!("{:?}", user);
        }
        return;
    }

    if show_user.id.is_none() && show_user.name.is_none() && show_user.email.is_none() {
        eprintln!("No query given. Please provide an ID, name, or email");
        return;
    }

    let user_query = UserQuery {
        id: show_user.id,
        name: show_user.name,
        email: show_user.email,
    };

    let found_users = find_users(&users, &user_query);

    if let Err(FindError::NoUserFound) = found_users {
        eprintln!("No user found from given query.");
        return;
    }

    let found_users = found_users.unwrap();

    for user in found_users {
        println!("{:?}", user);
    }
}
