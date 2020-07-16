use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::home_dir;

const CONFIG_HOME: &'static str = "XDG_CONFIG_HOME";
const CACHE_HOME: &'static str = "XDG_CACHE_HOME";
const DATA_HOME: &'static str = "XDG_DATA_HOME";

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

pub fn cache_home() -> Option<PathBuf>
{
    get_xdg_path(CACHE_HOME, ".cache")
}

pub fn data_home() -> Option<PathBuf>
{
    get_xdg_path(DATA_HOME, Path::new(".local").join("share"))
}