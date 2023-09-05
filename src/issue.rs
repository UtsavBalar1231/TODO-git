use crate::todo_git::TodoGit;
use crate::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// `Issue` is stripped json schema of the Github issues API, it stores the
/// title, number and body of all the issues of the specific GitHub repository.
///
/// `Issue` implements Serialization and Deserialization to extract only the
/// parts of complete GitHub issues JSON schema.
#[derive(Deserialize, Serialize)]
pub struct Issue {
    /// Handle to store the title of the issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// State of the issue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Handle to store the number of the issue from the list of issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<usize>,
    /// Handle to store the body content of the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Handle to issue id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl Issue {
    /// Create a new issue, with optional title and body arguments
    pub async fn create_new(
        client: &reqwest::Client,
        title: Option<&str>,
        body: Option<&str>,
    ) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let todo_git = TodoGit::parse(None).expect("Failed to parse todo_git config");

        let issues_url = format!(
            "https://api.github.com/repos/{}/{}/issues",
            todo_git.owner, todo_git.repo
        );

        client
            .post(&issues_url)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", todo_git.token))
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .json(&json!({
                "title": title.unwrap_or(""),
                "body": body.unwrap_or(""),
            }))
            .send()
            .await
    }

    /// Return the entire list of GitHub issues from the owner's GitHub
    /// repository.
    pub async fn get_issues(client: &reqwest::Client) -> Result<String, reqwest::Error> {
        let todo_git = TodoGit::parse(None).expect("Failed to parse todo_git config");

        let issues_url = format!(
            "https://api.github.com/repos/{}/{}/issues?state=all",
            todo_git.owner, todo_git.repo
        );

        client
            .get(&issues_url)
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send()
            .await
            .unwrap()
            .text()
            .await
    }

    /// Updates the body of a specific issue on the Owner's GitHub
    /// Repository and return the response back to the caller.
    ///
    /// This requires authentication. So, the `GITHUB_TOKEN` variable is set in
    /// the `todo_list.json` config which is located on the local storage of
    /// the User.
    pub async fn update_issue(
        &self,
        client: &reqwest::Client,
        body: &String,
    ) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let todo_git = TodoGit::parse(None).expect("Failed to parse todo_git config");
        let update_issue_url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}",
            todo_git.owner,
            todo_git.repo,
            self.number.unwrap()
        );

        client
            .patch(&update_issue_url)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", todo_git.token))
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .json(&json!({
                "body": body,
            }))
            .send()
            .await
    }

    /// Closes the latest issue from the Github repository
    pub async fn delete_issue(
        &self,
        client: &reqwest::Client,
    ) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let todo_git = TodoGit::parse(None).expect("Failed to parse todo_git config");

        // Construct the GraphQL query
        let query = format!(
            r#"
        mutation {{
            deleteIssue(input: {{issueId: {:?}, clientMutationId: "issue delete"}}) {{
                clientMutationId
            }}
        }}
    "#,
            self.node_id.as_ref().unwrap()
        );

        client
            .post("https://api.github.com/graphql")
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", todo_git.token))
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .json(&json!({ "query": query }))
            .send()
            .await
    }

    /// Returns a vector of `Issue` type
    pub async fn get_issues_list(client: &reqwest::Client) -> Result<Vec<Self>, serde_json::Error> {
        let get_issues = Self::get_issues(client)
            .await
            .expect("Failed to get issues");
        serde_json::from_str(&get_issues)
    }
}
