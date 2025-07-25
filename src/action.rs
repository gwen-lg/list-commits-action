use ghactions::prelude::*;

#[derive(Actions, Debug)]
#[action(
    name = "List Commits",
    description = "Generate the list of commits from an event (push or PR)",
    path = "./action.yml"
)]
pub struct ListCommitsAction {
    // Inputs & Outputs go here
}
