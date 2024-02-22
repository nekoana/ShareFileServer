use share_file_server::start_server;
use std::{env, fs};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: {} <port> <path> ", args[0]);
    } else {
        let path = args[2].as_str();

        let metadata = fs::metadata(path)?;
        if !metadata.is_dir() {
            panic!("path is not a directory");
        }

        let path = std::path::Path::new(path);

        println!("port:{} path: {}", args[1], path.display());

        let port = args[1].parse::<u16>().expect("port must be a number");

        start_server(path, port,async {
            tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
        }).await?;
    }

    Ok(())
}
