use crossterm::{cursor, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Write};

// Type alias
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn scraper(dump_to_stdout: bool, debug: bool) -> Result<()> {
    // // for each provider execute get_proxies
    // let mut stdout = stdout();

    // stdout.execute(cursor::Hide).unwrap();
    // for provider in providers {
    //     stdout.queue(cursor::SavePosition).unwrap();
    //     stdout
    //         .write_all(format!("🔎 {}", provider.name).as_bytes())
    //         .unwrap();
    //     // fetch sources from provider.sources (TOML)
    //     let sources = provider.sources;
    //     // get proxies from sources
    //     let _proxies = get_proxies(&client, sources, debug).await?;
    //     stdout.queue(cursor::RestorePosition).unwrap();
    //     stdout.flush().unwrap();
    //     stdout
    //         .write_all(format!("✔️ {}", provider.name).as_bytes())
    //         .unwrap();
    // }
    // stdout.execute(cursor::Show).unwrap();

    Ok(())
}
