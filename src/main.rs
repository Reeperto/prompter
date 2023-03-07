use display::PromptError;

mod display;

fn main() -> Result<(), PromptError> {
    display::prompt()?;
    Ok(())
}
