use std::path::{Path, PathBuf};

use crate::home_dir;

const CONFIG_HOME: &'static str = "XDG_CONFIG_HOME";
const CACHE_HOME: &'static str = "XDG_CACHE_HOME";
const DATA_HOME: &'static str = "XDG_DATA_HOME";

pub enum Directory
{
  Config,
  Cache,
  Data,
}

pub fn get_directory<P: AsRef<Path>>(dir: Directory, suffix: P) -> Option<PathBuf>
{
  let key = match dir {
    Directory::Config => CONFIG_HOME,
    Directory::Cache => CACHE_HOME,
    Directory::Data => DATA_HOME,
  };
  std::env::var_os(key)
    .and_then(|p| {
      if p.is_empty() {
        home_dir().and_then(|h| {
          Some(h.join(match dir {
            Directory::Config => ".config",
            Directory::Cache => ".cache",
            Directory::Data => ".local/share",
          }))
        })
      } else {
        Some(PathBuf::from(p))
      }
    })
    .map(|p| p.join(suffix))
}
