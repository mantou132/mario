extern crate clap;
use clap::{App, Arg};

mod serve;
mod github;

fn main() {
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
    
    let root = matches.value_of("root").unwrap_or("~/");
    let port = matches.value_of("port").unwrap_or("6464");

    serve::run(root, port);
}