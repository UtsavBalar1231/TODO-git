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

use std::{env, path};

/// Checks if a executable command exists on the Filesystem.
/// Return the full path of the executable if it exists.
pub fn find_command<P>(executable: P) -> Option<path::PathBuf>
where
    P: AsRef<path::Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&executable);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}
