use clap::Parser;
use display::PromptError;

mod display;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, default_value_t=false)]
    right: bool
}

fn main() -> Result<(), PromptError> {
    let args = Args::parse();

    // NOTE: Is this the best idea?
    // Potentially unify into a single command and do terminal cursor movements to handle left and
    // right prompting
    if args.right {
        display::right_prompt()
    } else {
        display::left_prompt()
    }

}
