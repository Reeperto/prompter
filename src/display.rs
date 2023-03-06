use crossterm::terminal;
use std::env;
use std::usize;

const BOLD: &str = "\u{001b}[1m";
const RESET: &str = "\u{001b}[0m";

const MAGENTA: &str = "\u{001b}[35m";
const GREEN: &str = "\u{001b}[32m";

const BRIGHT_BLACK: &str = "\u{001b}[38;2;38;38;38m";

// TODO: Programatically determine the longest path size
const MAX_PATH_LENGTH: usize = 30;

pub fn prompt() {
    let (term_width, _) = terminal::size().unwrap();

    let user = get_user();
    let cwd = get_cwd();
    let time = get_time();

    // TODO: Investigate fish_right_prompt
    let pad = "‧".repeat((term_width as usize) - user.len() - cwd.len() - time.len() - 4);

    let git = match in_git_repo() {
        true => " ",
        false => "",
    };

    let content_line = BOLD.to_owned()
        + MAGENTA
        + &user
        + RESET
        + GREEN
        + " "
        + &cwd
        + BRIGHT_BLACK
        + " "
        + &pad
        + " "
        + GREEN
        + &time;

    print!("{}", content_line);

    // TODO: Make sure that username length wont result in a kernel panic
    print!(
        "\n┗{}{}{RESET} ",
        git,
        "━".repeat(get_user().len() - git.chars().count() - 1)
    );
}

fn get_user() -> String {
    env::var("USER").unwrap()
}

fn get_time() -> String {
    let now = chrono::Local::now();

    now.format("%I:%M %p").to_string()
}

fn in_git_repo() -> bool {
    let cwd = env::current_dir().unwrap();

    return cwd.ancestors().any(|d| {
        d.read_dir()
            .unwrap()
            .any(|f| f.as_ref().unwrap().file_name().eq(".git"))
    });
}

fn get_cwd() -> String {
    let home_dir = env::var("HOME").unwrap();
    let cwd = env::current_dir();

    let dir = match cwd {
        Ok(cwd) => cwd.to_str().unwrap().to_string().replace(&home_dir, "~"),
        Err(_) => "[ERROR]".to_string(),
    };

    if dir.len() <= MAX_PATH_LENGTH {
        dir
    } else {
        let split_dir: Vec<&str> = dir.split('/').collect();
        let mut dir = String::new();

        (0..split_dir.len() - 1).for_each(|i| {
            if split_dir[i] != "~" {
                dir.push('/')
            }
            dir.push_str(split_dir[i].chars().next().unwrap().to_string().as_str());
        });
        dir.push('/');
        dir.push_str(split_dir[split_dir.len() - 1]);

        dir
    }
}
