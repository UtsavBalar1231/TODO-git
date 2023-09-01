/// Set default `USER_AGENT` to be this crate itself.
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Define LATEST_ISSUE as the first issue that is currently open on the
/// Git repository. This is required because the `get_issues` always return a
/// Vector of `Issue`. And we need the first issue to be modified.
///
/// TODO: Make it dynamic so that user can choose whichever issue he might want
/// to edit.
pub const LATEST_ISSUE: usize = 0;

pub mod issue;
pub mod todo_git;
