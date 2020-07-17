use std::path::PathBuf;

use di_rs::{self, xdg};

fn into_str(p: Option<PathBuf>) -> String
{
    p.and_then(|p| Some(format!("{}", p.display())))
        .unwrap_or(String::from("None"))
}

fn main()
{
    println!("home_dir() -> {}", into_str(di_rs::home_dir()));
    println!();
    println!(
        "xdg::get_directory(Config) -> {}",
        into_str(xdg::get_directory(xdg::Directory::Config, ""))
    );
    println!(
        "xdg::get_directory(Cache) --> {}",
        into_str(xdg::get_directory(xdg::Directory::Cache, ""))
    );
    println!(
        "xdg::get_directory(Data) ---> {}",
        into_str(xdg::get_directory(xdg::Directory::Data, ""))
    );
}
