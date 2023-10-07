use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use anstream::println;
use owo_colors::OwoColorize as _;

pub struct Args {
    pub remote_host: String,
    pub remote_user: String,
}

pub struct Config {
    ssh_key_comment: &str,
    ssh_key_path: &str,
}

impl Config {
    pub fn build(args: Args) -> Result<Config, Box<dyn Error>> {
        let remote_host = args.remote_host;
        let remote_user = args.remote_user;

        let local_host = env::var("HOSTNAME")?;
        let local_user = env::var("USER")?;

        let mut ssh_key_path = PathBuf::from(env::var("HOME")?);
        ssh_key_path.push(".ssh");
        let filename = format!("{remote_host}_{remote_user}");
        ssh_key_path.push(filename);

        let mut ssh_pub_key_path = ssh_key_path.clone();
        ssh_pub_key_path.set_extension("pub");

        let ssh_key_comment = format!("{local_user}@{local_host} for {remote_user}@{remote_host}");
        let ssh_key_comment = ssh_key_comment.as_str();
        let ssh_key_path = ssh_key_path.to_str()?;

        Ok(Config {
            ssh_key_comment,
            ssh_key_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!();
    let _ = Command::new("ssh-keygen")
        .args(["-t", "ed25519"])
        .args(["-C", config.ssh_key_comment])
        .args(["-f", config.ssh_key_path])
        .status()?;

    println!();
    let _ = Command::new("ssh-add").arg(config.ssh_key_path).status()?;

    Ok(())
}
