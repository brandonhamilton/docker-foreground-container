use signal_hook::iterator::Signals;
use std::env;
use std::process::exit;
use std::process::Command;
use time::{format_description::well_known::iso8601, OffsetDateTime};

fn run_command(cmd: &str, name: &str) -> anyhow::Result<String> {
    Command::new("docker")
        .args([cmd, name])
        .output()
        .map_err(From::from)
        .and_then(|output| {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                anyhow::bail!(String::from_utf8_lossy(&output.stderr).to_string())
            }
        })
}

fn now() -> String {
    OffsetDateTime::now_utc()
        .format(&iso8601::Iso8601::DEFAULT)
        .unwrap()
}

fn run_forever() -> anyhow::Result<String> {
    let mut sigs = Signals::new(signal_hook::consts::TERM_SIGNALS)?;
    // Terminate on the first signal received
    if let Some(signal) = (&mut sigs).into_iter().next() {
        return Ok(format!("Received termination signal {:?}", signal));
    }
    unreachable!()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <container_name>", env!("CARGO_PKG_NAME"));
        exit(-1)
    }
    println!("[{}] Starting container {}", now(), &args[1]);
    if let Err(e) = run_command("start", &args[1]) {
        eprintln!("[{}] {}", now(), e);
        exit(-1);
    };
    println!("[{}] {} is running", now(), &args[1]);
    match run_forever() {
        Ok(output) => {
            println!("[{}] {}", now(), &output);
        }
        Err(e) => {
            eprintln!("[{}] Failed to register signal handlers: {}", now(), e);
        }
    }
    println!("[{}] Stopping container {}", now(), &args[1]);
    if let Err(e) = run_command("stop", &args[1]) {
        eprintln!("{}", e);
        exit(-1);
    };
    println!("[{}] {} is no longer running", now(), &args[1]);
    Ok(())
}
