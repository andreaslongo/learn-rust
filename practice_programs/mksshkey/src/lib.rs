use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anstream::println;
use owo_colors::OwoColorize as _;

pub struct Args {
    pub remote_host: String,
    pub remote_user: String,
}

pub struct Config {
    ssh_key_comment: String,
    ssh_key_path: String,
    remote_host: String,
    remote_user: String,
}

impl Config {
    pub fn build(args: Args) -> Result<Config, Box<dyn Error>> {
        let remote_host = args.remote_host;
        let remote_user = args.remote_user;
        let local_host = env::var("HOSTNAME").unwrap_or(String::from("unknown-host"));
        let local_user = env::var("USER").unwrap_or(String::from("unknown-user"));

        let ssh_key_comment = format!("{local_user}@{local_host} for {remote_user}@{remote_host}");

        let mut ssh_key_path = PathBuf::from(env::var("HOME")?);
        ssh_key_path.push(".ssh");
        let filename = format!("{remote_host}_{remote_user}");
        ssh_key_path.push(filename);
        let ssh_key_path = String::from(ssh_key_path.to_str().expect("invalid ssh key path"));

        Ok(Config {
            ssh_key_comment,
            ssh_key_path,
            remote_host,
            remote_user,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!();
    let _ = Command::new("ssh-keygen")
        .args(["-t", "ed25519"])
        .args(["-C", &config.ssh_key_comment])
        .args(["-f", &config.ssh_key_path])
        .status()?;

    println!();
    let _ = Command::new("ssh-add").arg(&config.ssh_key_path).status()?;

    let ssh_pub_key_path = config.ssh_key_path.clone() + ".pub";
    let ssh_pub_key = fs::read_to_string(ssh_pub_key_path).unwrap();
    let ssh_pub_key = ssh_pub_key.trim();

    print_usage(config, ssh_pub_key);

    Ok(())
}

fn print_usage(config: Config, ssh_pub_key: &str) {
    let (remote_host, remote_user, ssh_key_path) =
        (config.remote_host, config.remote_user, config.ssh_key_path);

    let ssh_config = format!(
        "Host *
    PreferredAuthentications publickey
    IdentitiesOnly yes
    StrictHostKeyChecking yes
    UpdateHostKeys no
    Compression yes

Host {remote_host}
    Hostname {remote_host}
    User {remote_user}
    IdentityFile {ssh_key_path}"
    );

    let ssh_test = format!("ssh {remote_host}");

    let ssh_pub_key = ssh_pub_key.blue();
    let ssh_config = ssh_config.blue();
    let ssh_test = ssh_test.blue();
    let heading = "== Using the key".bold();
    let sep = "----".blue();

    println!(
        "
{heading}

Add the public key to the 'authorized_keys' file on the remote host:

{sep}
{ssh_pub_key}
{sep}

Define a new alias in your local SSH 'config' file:

{sep}
{ssh_config}
{sep}

Test the connection:

{sep}
{ssh_test}
{sep}
"
    );
}
