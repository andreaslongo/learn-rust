use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut args = env::args();
    args.next();
    let remote_host = match args.next() {
        Some(arg) => arg,
        // None => return Err("Didn't get a remote host"),
        None => panic!("Didn't get a remote host"),
    };

    let remote_user = match args.next() {
        Some(arg) => arg,
        // None => return Err("Didn't get a remote user"),
        None => panic!("Didn't get a remote user"),
    };

    let local_user = env::var("USER").unwrap_or(String::from("unknown-local-user"));
    let local_host = env::var("HOSTNAME").unwrap_or(String::from("unknown-local-host"));

    let ssh_key_comment = format!("{local_user}@{local_host} for {remote_user}@{remote_host}");
    dbg!(&ssh_key_comment);

    let mut ssh_key_path = PathBuf::from(env::var("HOME").unwrap_or(String::from(".")));
    if ssh_key_path.to_str().expect("invalid path") != "." {
        ssh_key_path.push(".ssh");
        fs::create_dir_all(&ssh_key_path).expect("failet to create SSH directory");
    }

    let filename = format!("{remote_host}_{remote_user}");
    ssh_key_path.push(filename);
    dbg!(&ssh_key_path);

    let ssh_key_comment = ssh_key_comment.as_str();
    let ssh_key_path = ssh_key_path.to_str().expect("invalid path");
    let status = Command::new("ssh-keygen")
        .args(["-t", "ed25519"])
        .args(["-C", ssh_key_comment])
        .args(["-f", ssh_key_path])
        .status()
        .expect("failed to execute 'ssh-keygen'");

    dbg!(&status);
}
