use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use structopt::StructOpt;
use toml;

/// Set a proxy for the other CLI
#[derive(StructOpt)]
struct CLI {
    program: String,
    url: String,
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

    let config = read_config();

    if let Some(proxy) = config.get(&program) {
        for cmd in proxy.set.iter() {
            if cmd.contains("{url}") {
                let cmd = cmd.replace("{url}", url.as_str());
                run_command(cmd);
            }
        }
    } else {
        println!("Sorry. Program: `{}` is not supported current now", program);
    }
    // match program.as_str() {
    //     "npm" => {
    //         run_command(cmd);
    //     }
    //     "yarn" => {
    //         let cmd = format!("yarn config set proxy {}", url);
    //         run_command(cmd);
    //     }
    //     "git" => {
    //         let cmd1 = format!("git config http.proxy {}", url);
    //         run_command(cmd1);
    //         let cmd2 = format!("git config https.proxy {}", url);
    //         run_command(cmd2);
    //     }
    //     "cargo" => {
    //         println!("cargo")
    //     }
    // }
}

fn read_config() -> HashMap<String, Proxy> {
    let toml_string = fs::read_to_string("config.toml").expect("No config file found");
    let data: HashMap<String, Proxy> = toml::from_str(&toml_string).unwrap();

    data
}

#[cfg(target_os = "windows")]
fn run_command(cmd: String) {
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
