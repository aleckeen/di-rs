use std::env;
use std::path::{PathBuf, Path};

use crate::home_dir;
use std::ffi::OsStr;

const CONFIG_HOME: &'static str = "XDG_CONFIG_HOME";
const CACHE_HOME: &'static str = "XDG_CACHE_HOME";

fn get_xdg_path<K: AsRef<OsStr>, P: AsRef<Path>>(key: K, suffix: P) -> Option<PathBuf>
{
    env::var_os(key)
        .and_then(|p| {
            if p.is_empty() {
                home_dir()
                    .and_then(|h| {
                        Some(h.join(suffix))
                    })
            } else {
                Some(PathBuf::from(p))
            }
        })
}

pub fn config_home() -> Option<PathBuf>
{
    get_xdg_path(CONFIG_HOME, ".config")
}