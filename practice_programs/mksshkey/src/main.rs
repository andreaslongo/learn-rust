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

    let local_user = env::var("USER").expect("failed to get local user");
    let local_host = env::var("HOSTNAME").expect("failed to get local host");

    let ssh_key_comment = format!("{local_user}@{local_host} for {remote_user}@{remote_host}");
    dbg!(&ssh_key_comment);

    let mut ssh_key_path = PathBuf::from(env::var("HOME").expect("failed to get home path"));
    ssh_key_path.push(".ssh");
    fs::create_dir_all(&ssh_key_path).expect("failet to create SSH directory");
    let filename = format!("{remote_host}_{remote_user}");
    ssh_key_path.push(filename);
    dbg!(&ssh_key_path);

    let status = Command::new("ssh-keygen")
        .args(["-t", "ed25519"])
        .args(["-C", ssh_key_comment.as_str()])
        .args(["-f", ssh_key_path.to_str().expect("invalid path")])
        .status()
        .expect("failed to execute process");

    println!("process finished with: {status}");

    assert!(status.success());
}
