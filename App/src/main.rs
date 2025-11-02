use std::error::Error;
use std::process::Command;
use std::string::String;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let playlist_url = "https://www.youtube.com/playlist?list=PL-EWx9Mw_fia6Od7xe3w3EdYrIdEn_DQm"; 

    let output = Command::new("yt-dlp")
        .arg("--print")
        .arg("title")
        .arg("--flat-playlist")
        .arg("--skip-download")
        .arg(playlist_url)
        .output()?;

    if output.status.success() {
        let titles = String::from_utf8_lossy(&output.stdout);
        let title_list: Vec<&str> = titles.lines().collect();
        for title in title_list {
            println!("Video Title: {}", title);
        }
    } else {
        eprintln!("Error: {:?}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
