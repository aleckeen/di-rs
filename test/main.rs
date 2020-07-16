use std::path::PathBuf;

use di_rs;

fn into_str(p: Option<PathBuf>) -> String
{
    p.and_then(|p| Some(format!("{}", p.display())))
        .unwrap_or(String::from("None"))
}

fn main()
{
    println!();
    println!(
        "di_rs::home_dir() ---------> {}",
        into_str(di_rs::home_dir())
    );
    println!(
        "di_rs::xdg::config_home() -> {}",
        into_str(di_rs::xdg::config_home())
    );
    println!(
        "di_rs::xdg::cache_home() --> {}",
        into_str(di_rs::xdg::cache_home())
    );
    println!(
        "di_rs::xdg::data_home() ---> {}",
        into_str(di_rs::xdg::data_home())
    );
    println!();
}
