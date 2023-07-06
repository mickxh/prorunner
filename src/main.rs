use std::process::Command;
use serde::Deserialize;
use toml;
use clap::Parser;

type Processes = Vec<Process>;

/// Prorunner CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the config file
    #[arg(short, long)]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    #[allow(dead_code)]
    processes: Processes,
}

#[derive(Debug, Deserialize)]
struct Process {
    #[allow(dead_code)]
    command: String,
}

fn main() {
    let args = Args::parse();
    
    let conf_path = args.config;
    let conf_str= std::fs::read_to_string(conf_path).unwrap();
    let conf: Config = toml::from_str(&conf_str).unwrap();
    
    for proc in conf.processes {
        run_process(proc);
    }
}

fn run_process(proc: Process) {
    let mut child = Command::new(&proc.command)
        .spawn()
        .unwrap();

    match child.try_wait() {
        Ok(Some(status)) => println!("exited with: {}", status),
        Ok(None) => {
            println!("still running");
            // child.kill().unwrap();
        }
        Err(e) => println!("error attempting to wait: {}", e),
    }
}
