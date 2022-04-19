use structopt::StructOpt;
use std::process::Command;

/// Set a proxy for the other CLI
#[derive(StructOpt)]
struct CLI {
    program: String,
    url: String,
}

fn main() {
    let args = CLI::from_args();
    let program = args.program;
    let url = args.url;

    match program.as_str() {
        "npm" => {
            let cmd = format!("npm config set proxy {}", url);
            run_command(cmd);
        },
        "yarn" => {
            let cmd = format!("yarn config set proxy {}", url);
            run_command(cmd);
        },
        "git" => {
            let cmd1 = format!("git config http.proxy {}", url);
            run_command(cmd1);
            let cmd2 = format!("git config https.proxy {}", url);
            run_command(cmd2);
        },
        "cargo" => {
            println!("cargo")
        },
        _ => println!("Sorry. Program: `{}` is not supported current now", program),
    }
}

#[cfg(target_os = "windows")]
fn run_command(cmd: String) {
    let mut args: Vec<&str> = cmd.split_whitespace().collect();
    args.insert(0, "/c");
    
    Command::new("cmd").args(args).output().expect("failed to execute process");
}

#[cfg(not(target_os = "windows"))]
fn run_command(cmd: String) {
    let mut args: Vec<&str> = cmd.split_whitespace().collect();
    let cmd = args[0];
    args.remove(0);
    
    Command::new(cmd).args(args).output().expect("failed to execute process");
}
