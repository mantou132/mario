use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod serve;
mod tools;

#[derive(Debug)]
pub struct Config {
    root: String,
    exec: String,
}

mod global {
    use super::Config;
    use once_cell::sync::OnceCell;

    pub static CONFIG: OnceCell<Config> = OnceCell::new();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 工作目录
    #[clap(default_value = "./")]
    root: String,

    /// 服务端口
    #[clap(short, long, env = "PORT", default_value_t = 6464)]
    port: u16,

    /// 更新仓库后执行的脚本
    #[clap(short, long, default_value_t)]
    exec: String,
}

fn main() {
    let Args { root, exec, port } = Args::parse();

    let root = PathBuf::from(root);
    let root = fs::canonicalize(&root)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    global::CONFIG.set(Config { root, exec }).unwrap();
    serve::run(port).unwrap();
}
