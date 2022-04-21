use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use toml;

/// Set a proxy for the other CLI
#[derive(StructOpt, Debug)]
struct CLI {
    /// Program wants to proxy
    program: String,

    /// Proxy URL
    #[structopt(name = "URL", required_unless("delete"))]
    url: Option<String>,

    /// Delete the proxy
    #[structopt(name = "delete", short = "d", long)]
    delete: bool,

    #[structopt(short, long, parse(from_os_str), default_value = "config.toml")]
    config: PathBuf
}

#[derive(Debug, Deserialize)]
struct Proxy {
    set: Vec<String>,
    unset: Vec<String>,
}

fn main() {
    let args = CLI::from_args();
    let program = args.program;
    let url = args.url;
    let delete = args.delete;
    let config_path = args.config;

    let config = read_config(config_path);

    if let Some(proxy) = config.get(&program) {
        match delete {
            true => {
                for cmd in proxy.unset.iter() {
                    run_command(cmd.clone());
                }
            }
            false => {
                if let Some(url) = url {
                    for cmd in proxy.set.iter() {
                        if cmd.contains("{url}") {
                            let cmd = cmd.replace("{url}", url.as_str());
                            run_command(cmd);
                        }
                    }
                }
            }
        }
    } else {
        println!("Sorry. Program: `{}` is not supported current now", program);
    }
}

fn read_config(path: PathBuf) -> HashMap<String, Proxy> {
    let toml_string = fs::read_to_string(path).expect("No config file found");
    let data: HashMap<String, Proxy> = toml::from_str(&toml_string).unwrap();

    data
}

#[cfg(target_os = "windows")]
fn run_command(cmd: String) {
    println!("Running: {}", cmd);
    let mut args: Vec<&str> = cmd.split_whitespace().collect();
    args.insert(0, "/c");

    Command::new("cmd")
        .args(args)
        .output()
        .expect("failed to execute process");
}

#[cfg(not(target_os = "windows"))]
fn run_command(cmd: String) {
    let mut args: Vec<&str> = cmd.split_whitespace().collect();
    let cmd = args[0];
    args.remove(0);

    Command::new(cmd)
        .args(args)
        .output()
        .expect("failed to execute process");
}
