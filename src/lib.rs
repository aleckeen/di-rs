use std::env;
use std::path::PathBuf;

pub mod xdg;

const HOME: &'static str = "HOME";

pub fn home_dir() -> Option<PathBuf>
{
    env::var_os(HOME)
        .and_then(|p| {
            if p.is_empty() {
                None
            } else {
                Some(PathBuf::from(p))
            }
        })
}

#[cfg(test)]
mod tests
{
    use std::path::PathBuf;

    use super::{home_dir, xdg};

    fn into_str(p: Option<PathBuf>) -> String
    {
        p.and_then(|p| Some(format!("{}", p.display()))).unwrap_or(String::from("None"))
    }

    #[test]
    fn print_all()
    {
        println!();
        println!("home_dir() ---------> {}", into_str(home_dir()));
        println!("xdg::config_home() -> {}", into_str(xdg::config_home()));
        println!("xdg::cache_home() --> {}", into_str(xdg::cache_home()));
        println!("xdg::data_home() ---> {}", into_str(xdg::data_home()));
        println!();
    }
}