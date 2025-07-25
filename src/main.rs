use ghactions::prelude::*;

use anyhow::Result;
use log::info;

mod action;

fn main() -> Result<()> {
    let mut action = action::ListCommitsAction::init()?;
    info!("Starting action: {}", action.name());

    // Your code goes here
    action.set_commits_list(vec!["todo".into()]);

    Ok(())
}
