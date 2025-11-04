use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let playlist_url = "https://www.youtube.com/playlist?list=PL-EWx9Mw_fiZqdOhQB_oU8ut68nYLLqzP";
    let archive_file = "downloaded.txt"; // Arquivo para trackear downloads

    println!("🎵 Iniciando download massivo de playlist");
    println!("💾 Usando arquivo de checkpoint: {}", archive_file);

    let mut cmd = Command::new("yt-dlp");

    cmd.arg("--ignore-errors")
        .arg("--continue")
        .arg("--no-overwrites")
        .arg("--no-part")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--audio-quality")
        .arg("0")
        .arg("--embed-thumbnail")
        .arg("--add-metadata")
        .arg("--concurrent-fragments")
        .arg("5")
        .arg("--limit-rate")
        .arg("1M")
        .arg("--sleep-interval")
        .arg("2")
        .arg("--newline")
        .arg("--progress")
        .arg("-o")
        .arg("Music/%(title)s.%(ext)s");

    // Adiciona arquivo de archive se existir, ou cria um novo
    if fs::metadata(archive_file).is_ok() {
        println!("🔄 Continuando download anterior...");
        cmd.arg("--download-archive").arg(archive_file);
    } else {
        println!("🚀 Iniciando novo download...");
        cmd.arg("--download-archive").arg(archive_file);
    }

    cmd.arg(playlist_url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let start_time = Instant::now();
    let mut child = cmd.spawn()?;

    // Processamento eficiente da saída
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        tokio::task::spawn_blocking(move || {
            let mut count = 0;
            for line in reader.lines() {
                if let Ok(line) = line {
                    count += 1;
                    if count % 5 == 0 {
                        // Atualiza a cada 5 músicas
                        let elapsed = start_time.elapsed().as_secs_f64() / 60.0;
                        println!("📦 Baixadas: {} | Tempo: {:.1}min", count, elapsed);
                    }
                    if line.contains("100%") {
                        println!("✅ {}", line);
                    }
                }
            }
        });
    }

    let status = child.wait()?;

    if status.success() {
        println!("✨ Download concluído! Verifique a pasta 'Music/'");
    }

    Ok(())
}
