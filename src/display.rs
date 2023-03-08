use std::env::VarError;
use std::io::{stdout, Write};
use std::usize;
use std::{env, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PromptError {
    #[error("Unable to display content")]
    DisplayError(#[from] io::Error),
    #[error("Unable to get environment key `{0}`")]
    GatherError(#[from] VarError),
}

const BOLD: &str = "\u{001b}[1m";
const RESET: &str = "\u{001b}[0m";

const MAGENTA: &str = "\u{001b}[35m";
const GREEN: &str = "\u{001b}[32m";

const BRIGHT_BLACK: &str = "\u{001b}[38;2;38;38;38m";

// TODO: Programatically determine the longest path size
const MAX_PATH_LENGTH: usize = 30;

pub fn left_prompt() -> Result<(), PromptError> {
    let user = get_user()?;
    let cwd = get_cwd()?;

    // TODO: Investigate fish_right_prompt
    // NOTE: By placing characters within the space, under terminal resizes the prompt characters
    // wrap around and in general makes it look quite ugly. *Potentially remove*.

    let git = match in_git_repo() {
        true => " ",
        false => "",
    };

    // TODO: Investigate doing a modular system
    let content_line =
        BOLD.to_owned() + MAGENTA + &user + RESET + GREEN + " " + &cwd + BRIGHT_BLACK;

    stdout().write_all(&content_line.into_bytes())?;

    // TODO: Make sure that username length wont result in a panic
    let content_line = GREEN.to_string()
        + "\n┗"
        + git
        + &"━".repeat(user.len() - git.chars().count() - 1)
        + RESET
        + " ";

    stdout().write_all(&content_line.into_bytes())?;

    Ok(())
}

pub fn right_prompt() -> Result<(), PromptError> {
    let time = get_time();
    let content = GREEN.to_owned() + &time + RESET;

    stdout().write_all(&content.into_bytes())?;

    Ok(())
}

fn get_user() -> Result<String, VarError> {
    env::var("USER")
}

fn get_time() -> String {
    let now = chrono::Local::now();

    now.format("%I:%M %p").to_string()
}

fn in_git_repo() -> bool {
    // If the current directory fails to match, just safely return false
    let cwd = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return false,
    };

    // If getting a dir's contents fails, just ignore it
    cwd.ancestors().any(|d| match d.read_dir() {
        Ok(mut contents) => contents.any(|f| match f.as_ref() {
            Ok(file) => file.file_name().eq(".git"),
            Err(_) => false,
        }),
        Err(_) => false,
    })
}

fn get_cwd() -> Result<String, PromptError> {
    let home_dir = env::var("HOME")?;
    let cwd = env::current_dir()?;

    let dir = match cwd.to_str() {
        Some(dir) => dir.to_string().replace(&home_dir, "~"),
        None => "[ERROR]".to_string(),
    };

    if dir.len() <= MAX_PATH_LENGTH {
        Ok(dir)
    } else {
        let split_dir: Vec<&str> = dir.split('/').collect();
        let mut dir = String::new();

        (0..split_dir.len() - 1).for_each(|i| {
            if split_dir[i] != "~" {
                dir.push('/')
            }
            dir.push_str(
                split_dir[i]
                    .chars()
                    .next()
                    .unwrap_or(' ')
                    .to_string()
                    .as_str(),
            );
        });
        dir.push('/');
        dir.push_str(split_dir[split_dir.len() - 1]);

        Ok(dir)
    }
}
