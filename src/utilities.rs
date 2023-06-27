use std::io;

pub fn confirm(
    prompt: &str,
    post_prompt: Option<&str>,
    cancel_message: Option<&str>,
    default: Option<bool>,
) -> bool {
    println!(
        "{} [{}]es/[{}]o",
        prompt,
        if let Some(true) = default { "Y" } else { "y" },
        if let Some(false) = default { "N" } else { "n" }
    );

    if post_prompt.is_some() {
        println!("{}", post_prompt.unwrap());
    }

    let mut input = "".to_string();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.to_lowercase().trim() {
            "y" | "yes" => return true,
            "n" | "no" => {
                if cancel_message.is_some() {
                    println!("{}", cancel_message.unwrap());
                }
                return false;
            }
            "" => match default {
                Some(true) => return true,
                Some(false) => {
                    if cancel_message.is_some() {
                        println!("{}", cancel_message.unwrap());
                    }
                    return false;
                }
                None => {
                    eprintln!("Invalid input");
                    input = "".to_string();
                }
            },
            _ => {
                eprintln!("Invalid input");
                input = "".to_string();
            }
        }
    }
}
