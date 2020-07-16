use std::env;
use std::path::PathBuf;

pub mod xdg;

const HOME: &'static str = "HOME";

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
