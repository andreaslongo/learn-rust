use std::process;

use clap::Parser;

use mksshkey::Args;
use mksshkey::Config;

fn main() {
    let cli = Cli::parse();

    let args = Args {
        remote_host: cli.remote_host,
        remote_user: cli.remote_user,
    };

    let config = Config::build(args).unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}\nTry 'mksshkey --help' for more information.");
        process::exit(1);
    });

    if let Err(e) = mksshkey::run(config) {
        eprintln!("Application error: {e}\nTry 'mksshkey --help' for more information.");
        process::exit(1);
    }
}

/// A simple wrapper around OpenSSH to create SSH keys
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
#[command(
    help_template = "{about-section}\n{usage-heading} {usage}\n\n{all-args}\n\nWritten by {author}\nhttps://github.com/andreaslongo/mksshkey"
)]
pub struct Cli {
    /// The remote host for which the new key should be authorized
    remote_host: String,

    /// The remote user for which the new key should be authorized
    remote_user: String,
}
