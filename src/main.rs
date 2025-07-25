use ghactions::prelude::*;

use anyhow::{anyhow, bail, Context, Result};
use log::info;
use serde_json::Value;

mod action;
mod toolkit; // hack

fn main() -> Result<()> {
    let mut action = action::ListCommitsAction::init()?;
    info!("Starting action: {}", action.name());

    let github_event_name = action.get_event_name()?;
    info!("Started from `{github_event_name}`");

    let github_event = get_event()?;

    let base_ref = github_event
        .get("base_ref")
        .ok_or_else(|| anyhow!("failed to get `base_ref` value from push event"))?;
    info!("base_ref={base_ref}");

    info!("Started from `{github_event_name}`");
    let before = github_event
        .get("before")
        .ok_or_else(|| anyhow!("failed to get `before` value from push event"))?;
    info!("before={before}");

    let commits = github_event
        .get("commits")
        .ok_or_else(|| anyhow!("failed to get `commits` from push event"))?;

    let commits = if let Value::Array(commits) = commits {
        commits
            .iter()
            .map(|commit_obj| commit_obj.get("sha").unwrap().as_str().unwrap().to_string())
            .collect::<Vec<_>>()
    } else {
        bail!("commits are not an array")
    };

    // Your code goes here
    action
        .set_commits_list(commits)
        .context("set commits list output")?;

    Ok(())
}

fn get_event() -> Result<serde_json::Value> {
    let github_event_path =
        std::env::var("GITHUB_EVENT_PATH").context("Read env var `GITHUB_EVENT_PATH`")?;
    let event_content = std::fs::read_to_string(&github_event_path)
        .with_context(|| format!("Read event file `{github_event_path:?}`"))?;

    let root: serde_json::Value =
        serde_json::from_str(&event_content).context("Deserialise github event json")?;

    Ok(root)
}
