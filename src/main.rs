use clap::{Args, Parser, Subcommand};
use env_home::env_home_dir as home_dir;
use tracing_subscriber::prelude::*;

#[derive(Parser)]
#[command(name = "cli", about = "Yet another Node.js package manager")]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    #[command(name = "doctor", about = "Check that the configuration is correct")]
    Doctor(Doctor),
}

#[derive(Args, Debug)]
#[command(name = "doctor", about = "Check the health of the system")]
struct Doctor {
    // #[arg(short, long, help = "Number of times to check health")]
    // times: Option<u32>,
}

#[tracing::instrument]
fn main() {
    tracing_subscriber::Registry::default()
        .with(sentry::integrations::tracing::layer())
        .init();

    let _guard = sentry::init((
        "https://56b5cf6ec6d04220d1ba9dfbacb720dd@o1413557.ingest.us.sentry.io/4507903000576000",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    // implementation of main
    let cli = Cli::parse();

    match &cli.command {
        SubCommand::Doctor(args) => {
            doctor(args);
        }
    }

}

#[tracing::instrument]
fn doctor(_args: &Doctor) {
    let home = match home_dir() {
        Some(path) => path.display().to_string(),
        None => {
            eprintln!("Failed to get user home directory");
            std::process::exit(1);
        }
    };

    println!("Home: {}", home);
}

