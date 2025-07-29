use std::env::{self, VarError};

use action::ListCommitsAction;
use fallible_iterator::{FallibleIterator, convert};
use ghactions::prelude::*;

use anyhow::{Context, Error, Result, anyhow, bail};
use log::{debug, info};
use serde_json::Value;

mod action;

fn main() -> Result<()> {
    let mut action = ListCommitsAction::init()?;
    info!("Starting action: {}", action.name());

    let github_event_name = action.get_event_name()?;
    info!("Started from `{github_event_name}`");

    let github_event = get_event()?;
    //info!("GitHub Event : {github_event:#?}");

    //Debug env :
    debug_github_env();

    let commits = if github_event_name == "push" {
        push_commits(&github_event_name, &github_event)
    } else if github_event_name == "pull_request" {
        pull_request_commits(github_event)
    } else {
        Err(anyhow!("Event `{github_event_name}` is not handle"))
    }?;

    // Your code goes here
    action
        .set_commits_list(commits)
        .context("set commits list output")?;

    Ok(())
}

fn pull_request_commits(github_event: Value) -> Result<Vec<String>, Error> {
    let base_obj = github_event
        .get("base")
        .ok_or_else(|| anyhow!("failed to get `base` value from `pull_request` event"))?;
    let base_ref = base_obj
        .get("ref")
        .ok_or_else(|| anyhow!("failed to get `ref` in base object"))?;
    let base_sha = base_obj
        .get("sha")
        .ok_or_else(|| anyhow!("failed to get `sha` in base object"))?
        .to_string();
    let head_obj = github_event
        .get("head")
        .ok_or_else(|| anyhow!("failed to get `base` value from `pull_request` event"))?;
    let head_ref = head_obj
        .get("ref")
        .ok_or_else(|| anyhow!("failed to get `ref` in head object"))?;
    let head_sha = head_obj
        .get("sha")
        .ok_or_else(|| anyhow!("failed to get `sha` in head object"))?
        .to_string();
    debug!("Pull request with base `{base_ref}` and head `{head_ref}`");
    let commits = vec![head_sha, base_sha];
    Ok(commits)
}

fn push_commits(github_event_name: &String, github_event: &Value) -> Result<Vec<String>, Error> {
    let base_ref = github_event
        .get("base_ref")
        .ok_or_else(|| anyhow!("failed to get `base_ref` value from {github_event_name} event"))?;
    info!("base_ref={base_ref}");
    let before = github_event
        .get("before")
        .ok_or_else(|| anyhow!("failed to get `before` value from {github_event_name} event"))?;
    info!("before={before}");
    let commits = github_event
        .get("commits")
        .ok_or_else(|| anyhow!("failed to get `commits` from {github_event_name} event"))?;
    let commits = if let Value::Array(commits) = commits {
        convert(commits.iter().map(|commit_obj| {
            commit_obj
                .get("id")
                .ok_or_else(|| anyhow!("Can't get `id` element (commit sha) in :\n{commit_obj:#?}"))
        }))
        .map(|commit_sha| {
            commit_sha
                .as_str()
                .ok_or_else(|| anyhow!("failed to convert {commit_sha:#?}"))
                .map(|std| std.to_string())
        })
        .collect::<Vec<_>>()?
    } else {
        bail!("commits are not an array")
    };
    Ok(commits)
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

fn debug_env_var(var: &str) {
    match env::var(var) {
        Ok(value) => info!("{var}: {value}"),
        Err(VarError::NotPresent) => warn!("{var} is not present"),
        Err(VarError::NotUnicode(_)) => warn!("{var} have not unicode value"),
    }
}
fn debug_github_env() {
    // GITHUB_JOB
    debug_env_var("GITHUB_REF");
    debug_env_var("GITHUB_SHA");
    // GITHUB_REPOSITORY
    // GITHUB_REPOSITORY_OWNER
    // GITHUB_REPOSITORY_OWNER_ID
    // GITHUB_RUN_ID
    // GITHUB_RUN_NUMBER
    // GITHUB_RETENTION_DAYS
    // GITHUB_RUN_ATTEMPT
    // GITHUB_ACTOR_ID
    // GITHUB_ACTOR
    // GITHUB_WORKFLOW
    debug_env_var("GITHUB_HEAD_REF");
    debug_env_var("GITHUB_BASE_REF");
    // GITHUB_EVENT_NAME
    // GITHUB_SERVER_URL
    // GITHUB_API_URL
    // GITHUB_GRAPHQL_URL
    debug_env_var("GITHUB_REF_NAME");
    // GITHUB_REF_PROTECTED
    debug_env_var("GITHUB_REF_TYPE");
    // GITHUB_WORKFLOW_REF
    // GITHUB_WORKFLOW_SHA
    // GITHUB_REPOSITORY_ID
    // GITHUB_TRIGGERING_ACTOR
    // GITHUB_WORKSPACE
    // GITHUB_ACTION
    debug_env_var("GITHUB_EVENT_PATH");
    // GITHUB_ACTION_REPOSITORY
    // GITHUB_ACTION_REF
    // GITHUB_PATH
    debug_env_var("GITHUB_ENV");
    // GITHUB_STEP_SUMMARY
    // GITHUB_STATE
    debug_env_var("GITHUB_OUTPUT");
    // RUNNER_OS
    // RUNNER_ARCH
    // RUNNER_NAME
    // RUNNER_ENVIRONMENT"
}
