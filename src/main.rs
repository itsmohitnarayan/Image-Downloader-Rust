use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

use log::{info};

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

fn generate_filename(url: &reqwest::Url) -> String {
    url.path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin")
        .to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init(); // Initialize logger

    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    let response = reqwest::get(target).await?;
    let fname = generate_filename(response.url());

    info!("file to download: '{}'", fname);
    let fname = tmp_dir.path().join(fname);
    info!("will be located under: '{:?}'", fname);

    let mut dest = File::create(fname)?;

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}
