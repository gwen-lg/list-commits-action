use ghactions::prelude::*;

use anyhow::Result;
use log::info;

mod action;

fn main() -> Result<()> {
    let action = action::ListCommitsAction::init()?;
    info!("Starting action: {}", action.name());

    // Your code goes here

    Ok(())
}
