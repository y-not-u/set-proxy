use serde_derive::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use structopt::StructOpt;
use toml;

/**
Set a proxy for the other CLI

Example:

Set: setproxy npm http://127.0.0.1:8080/

Unset: setproxy -d npm
*/
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
    config: PathBuf,
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
                    run_command(cmd.as_str());
                }
            }
            false => {
                if let Some(url) = url {
                    for cmd in proxy.set.iter() {
                        let cmd: String = match cmd.contains("{url}") {
                            true => cmd.replace("{url}", url.as_str()),
                            false => String::from(cmd),
                        };
                        run_command(cmd.as_str());
                    }
                }
            }
        }
    } else {
        println!("Sorry. Program: `{}` is not supported current now", program);
    }
}

fn read_config(path: PathBuf) -> HashMap<String, Proxy> {
    let mut config_path = path.clone();
    if config_path.exists() == false {
        let home_dir = env::home_dir().unwrap();
        config_path = [home_dir.to_str().unwrap(), ".config", "setproxy.toml"]
            .iter()
            .collect();
    }

    if config_path.exists() == false {
        eprintln!("Error: config file not found");
    }

    let toml_string = fs::read_to_string(config_path).expect("No config file found");
    let data: HashMap<String, Proxy> = toml::from_str(&toml_string).unwrap();

    data
}

#[cfg(target_os = "windows")]
fn run_command(cmd: &str) {
    println!("Running: {}", cmd);
    let mut args: Vec<&str> = cmd.split_whitespace().collect();
    args.insert(0, "/c");

    Command::new("cmd")
        .args(args)
        .output()
        .expect("failed to execute process");
}

#[cfg(not(target_os = "windows"))]
fn run_command(cmd: &str) {
    println!("Running: {}", cmd);
    let mut args: Vec<&str> = cmd.split_whitespace().collect();
    let cmd = args[0];
    args.remove(0);

    Command::new(cmd)
        .args(args)
        .output()
        .expect("failed to execute process");
}
