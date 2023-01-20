mod args;

use args::{Args, Parser};

use headless_chrome::{Browser, LaunchOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let browser = Browser::new(LaunchOptions {
        headless: args.headless,
        ..Default::default()
    })?;

    browser.wait_for_initial_tab()?;

    println!("{:?}", args);

    Ok(())
}
