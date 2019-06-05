#[macro_use]
extern crate json;
extern crate clap;
use std::env;
use std::io::Result;
use clap::{App, Arg};

mod serve;
mod tools;

fn main() -> Result<()>{
    let matches = App::new("mario")
        .version("0.0.1")
        .author("mantou <709922234@qq.com>")
        .about("simple CI/CD")
        .arg(
            Arg::with_name("root")
                .short("r")
                .value_name("PATH")
                .help("输入路径"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .value_name("PORT")
                .help("输入端口"),
        )
        .get_matches();
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.to_str();
    let root = matches.value_of("root").unwrap_or(match current_dir {
        Some(dir) => dir,
        None => panic!("get current dir fail"),
    });
    let port = matches.value_of("port").unwrap_or("6464");

    serve::run(root, port)?;
    Ok(())
}