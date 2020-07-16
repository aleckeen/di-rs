use std::env;
use std::path::PathBuf;

use crate::home_dir;

const CONFIG_HOME: &'static str = "XDG_CONFIG_HOME";

pub fn config_home() -> Option<PathBuf>
{
    env::var_os(CONFIG_HOME)
        .and_then(|p| {
            if p.is_empty() {
                home_dir()
                    .and_then(|h| {
                        Some(h.join(".config"))
                    })
            } else {
                Some(PathBuf::from(p))
            }
        })
}