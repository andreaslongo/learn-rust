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

    let mut ssh_key_path = PathBuf::from(env::var("HOME").unwrap_or(String::from(".")));
    if ssh_key_path.to_str().expect("invalid path") != "." {
        ssh_key_path.push(".ssh");
        fs::create_dir_all(&ssh_key_path).expect("failet to create SSH directory");
    }

    let filename = format!("{remote_host}_{remote_user}");
    ssh_key_path.push(filename);

    let mut ssh_pub_key_path = ssh_key_path.clone();
    ssh_pub_key_path.set_extension("pub");


    let ssh_key_comment = ssh_key_comment.as_str();
    let ssh_key_path = ssh_key_path.to_str().expect("invalid path");

    println!();
    let _ = Command::new("ssh-keygen")
        .args(["-t", "ed25519"])
        .args(["-C", ssh_key_comment])
        .args(["-f", ssh_key_path])
        .status()
        .expect("failed to execute 'ssh-keygen'");


    println!();
    let _ = Command::new("ssh-add")
        .arg(ssh_key_path)
        .status()
        .expect("failed to execute 'ssh-add'");


    let ssh_config = format!("Host *
    PreferredAuthentications publickey
    IdentitiesOnly yes
    StrictHostKeyChecking yes
    UpdateHostKeys no
    Compression yes

Host {remote_host}
    Hostname {remote_host}
    User {remote_user}
    IdentityFile {ssh_key_path}");

    let ssh_pub_key = fs::read_to_string(ssh_pub_key_path).unwrap();
    let ssh_pub_key = ssh_pub_key.trim();
    let usage = format!("
== Using the key

Add the public key to the 'authorized_keys' file on the remote host:

----
{ssh_pub_key}
----

Define a new alias in your local SSH 'config' file:

----
{ssh_config}
----

Test the connection:

----
ssh {remote_host}
----
");

    println!("{usage}")

}
