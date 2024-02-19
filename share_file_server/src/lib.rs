use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use axum::{
    extract::{OriginalUri, State},
    http::Uri,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn start_server(path: impl AsRef<Path>, port: u16) -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "share_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let path = path
        .as_ref()
        .canonicalize()
        .expect("not available path")
        .to_string_lossy()
        .to_string();

    let app = Router::new()
        .route("/", get(root_handler))
        .fallback(get(root_handler))
        .with_state(path.clone())
        .nest_service("/static", ServeDir::new(path))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        })
        .await?;

    Ok(())
}

async fn root_handler(
    State(base_path): State<String>,
    OriginalUri(uri): OriginalUri,
) -> impl IntoResponse {
    let sub_path = uri.path().trim_start_matches('/');

    let mut full_path = PathBuf::from(base_path);
    full_path.push(sub_path);

    debug!("full_path: {:?} ", full_path);

    let mut file_list = String::from("<ul>");

    let files = list_files(&full_path).await;

    match files {
        Ok(files) => {
            for file in files {
                let mut href = String::new();
                match &file {
                    FileOrDir::File(file) => {
                        href.push_str("/static/");
                        if !sub_path.is_empty() {
                            href.push_str(sub_path);
                            href.push_str("/");
                        }
                        href.push_str(file);
                    }
                    FileOrDir::Dir(file) => {
                        href.push_str("/");
                        if !sub_path.is_empty() {
                            href.push_str(sub_path);
                            href.push_str("/");
                        }
                        href.push_str(file);
                        href.push_str("/");
                    }
                };

                file_list.push_str(&format!(
                    "<li><a href='{}'>{}</a></li>",
                    href,
                    file.to_string()
                ));
            }
        }
        Err(e) => {
            file_list.push_str(&format!("<li>Error: {}</li>", e));
        }
    }

    file_list.push_str("</ul>");

    Html(file_list)
}

enum FileOrDir {
    File(String),
    Dir(String),
}

impl Into<String> for FileOrDir {
    fn into(self) -> String {
        match self {
            FileOrDir::File(file) => file,
            FileOrDir::Dir(dir) => dir,
        }
    }
}

impl FileOrDir {
    fn to_string(self) -> String {
        self.into()
    }
}

async fn list_files(path: &std::path::Path) -> std::io::Result<Vec<FileOrDir>> {
    let mut files = Vec::new();

    let mut dir = tokio::fs::read_dir(path).await?;

    while let Ok(Some(entry)) = dir.next_entry().await {
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            let file_name = file_name.to_string_lossy().to_string();
            if path.is_dir() {
                files.push(FileOrDir::Dir(file_name));
            } else {
                files.push(FileOrDir::File(file_name));
            }
        }
    }

    Ok(files)
}
