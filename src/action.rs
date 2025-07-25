use ghactions::prelude::*;

#[derive(Actions, Debug)]
#[action(
    name = "List Commits",
    description = "Generate the list of commits from an event (push or PR)",
    path = "./action.yml"
)]
pub struct ListCommitsAction {
    #[output(
        // Output Description
        description = "List of commits sha",
    )]
    commits: String,
}

impl ListCommitsAction {
    pub fn set_commits_list(&mut self, commits: Vec<String>) -> Result<(), serde_json::Error> {
        self.commits = serde_json::to_string(&commits)?;
        Ok(())
    }
}
