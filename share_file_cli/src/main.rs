use clap::{Parser};
use share_file_server::start_server;
use std::{fs};

#[derive(Parser, Debug)]
struct Config {
    /// listen port
    #[arg(long)]
    port: u16,
    /// share path
    #[arg(long)]
    path: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Config::parse();

    let path = args.path.as_str();

    let metadata = fs::metadata(path)?;
    if !metadata.is_dir() {
        panic!("path is not a directory");
    }

    let path = std::path::Path::new(path);

    let port = args.port;

    start_server(path, port, async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    })
    .await?;

    Ok(())
}
