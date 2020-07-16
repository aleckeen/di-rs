use std::env;
use std::path::PathBuf;

pub mod xdg;

const HOME: &'static str = "HOME";

/// Returns the path extracted from the `$HOME` variable. If
/// it is empty for some reason, `None` is returned.
pub fn home_dir() -> Option<PathBuf>
{
    env::var_os(HOME).and_then(|p| {
        if p.is_empty() {
            None
        } else {
            Some(PathBuf::from(p))
        }
    })
}
