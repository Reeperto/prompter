use std::env;
use std::usize;
use crossterm::terminal;

const BOLD: &str = "\u{001b}[1m";
const RESET: &str = "\u{001b}[0m";

const MAGENTA: &str = "\u{001b}[35m";
const GREEN: &str = "\u{001b}[32m";

const BRIGHT_BLACK: &str = "\u{001b}[38;2;38;38;38m";

const MAX_PATH_LENGTH: usize = 25;

fn main() {

    let mut content_line = String::new();

    let user = get_user();
    let cwd = get_cwd();

    content_line.push_str(BOLD);
    content_line.push_str(MAGENTA);
    content_line.push_str(&user);
    content_line.push_str(RESET);
    content_line.push_str(GREEN);
    content_line.push_str(" ");
    content_line.push_str(&cwd);

    let (term_width, _) = terminal::size().unwrap().into();
    let time = get_time();

    content_line.push_str(BRIGHT_BLACK);
    content_line.push_str(" ");
    let pad ="‧".repeat((term_width as usize) - user.len() - cwd.len() - time.len() - 4);
    content_line.push_str(&pad);
    content_line.push_str(" ");
    content_line.push_str(GREEN);

    content_line.push_str(&time);

    print!("{}", content_line);

    print!("\n┗{}{RESET} ", "━".repeat(get_user().len() - 1));
}

fn get_user() -> String {
    env::var("USER")
        .unwrap()
}

fn get_time() -> String {
    let now = chrono::Local::now();

    now.format("%I:%M %p").to_string()
}

fn get_cwd() -> String {
    let home_dir = env::var("HOME").unwrap();
    let cwd = env::current_dir();

    let dir = match cwd {
        Ok(cwd) => {
            cwd.into_os_string()
                .to_str()
                .unwrap()
                .to_string()
                .replace(&home_dir, "~")
        },
        Err(_) => {
            String::from("[ERROR]")
        }
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
            dir.push_str(
                split_dir[i].chars()
                .next()
                .unwrap()
                .to_string()
                .as_str()
            );
        });
        dir.push('/');
        dir.push_str(split_dir[split_dir.len() - 1]);

        dir
    }

}
