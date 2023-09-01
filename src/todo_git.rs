use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main struct that stores the GitHub username and the specific repository
/// for modification of the TODO issues.
///
/// `owner`: Represents the GitHub username.
///
/// `repo`: Represents the GitHub repository of the **owner**.
///
/// `token`: Represents the GitHub authentication token of the *owner*.
#[derive(Deserialize, Serialize, Default)]
pub struct TodoGit {
    /// Handle to store the GitHub owner or username.
    pub owner: String,
    /// Handle to store the GitHub owner's repository.
    pub repo: String,
    /// Handle to store the GitHub owner's token for authentication purposes.
    pub token: String,
}

impl TodoGit {
    /// Returns a deserialized `TodoGit` initialization from the
    /// **.todo_git.json** config stored on the local storage of the user.
    pub fn parse(path: Option<&str>) -> Result<Self, serde_json::Error> {
        let todo_config: Option<PathBuf>;

        if let Some(path) = path {
            todo_config = Some(PathBuf::from(path));
        } else {
            todo_config = Some(PathBuf::from(format!(
                "{}/.{}.json",
                env!("HOME"),
                env!("CARGO_PKG_NAME")
            )));

            if let Some(ref config_path) = todo_config {
                if !config_path.exists() {
                    println!("{} config file does not exist!", config_path.display());
                    std::process::exit(1);
                }
            }
        }

        serde_json::from_str(
            std::fs::read_to_string(todo_config.unwrap())
                .expect("Error reading file!")
                .as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_config = serde_json::to_string_pretty(&TodoGit::default()).unwrap();

        let config_path = "/tmp/.todo-git.json";
        let status = std::fs::write(config_path, test_config);

        assert!(status.is_ok());

        let status = TodoGit::parse(Some(config_path));

        if status.is_err() {
            std::fs::remove_file(config_path).unwrap();
        }
        assert!(status.is_ok());
    }
}
