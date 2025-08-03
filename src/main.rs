pub mod components;
use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let dir = env::current_dir()?;
    components::installer::install(&dir);

    Ok(())
}
