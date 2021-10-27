use std::env;

use async_std::task;
use eyre::Result;

mod response_struct;
mod router;

fn main() -> Result<()> {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port: u16 = env::var("PORT").unwrap_or("4000".to_string()).parse()?;
    task::block_on(start(&host, port))?;
    Ok(())
}

async fn start(host: &str, port: u16) -> Result<()> {
    let mut app = tide::new();
    app.at("/search/:mart/:keyword").get(router::search);
    app.at("/info/:mart/:name").get(router::info);
    app.listen(format!("{}:{}", host, port)).await?;
    Ok(())
}