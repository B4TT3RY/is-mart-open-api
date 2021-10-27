use async_std::task;
use eyre::Result;

mod router;

fn main() -> Result<()> {
    task::block_on(start())?;
    Ok(())
}

async fn start() -> Result<()> {
    let mut app = tide::new();
    app.at("/search/:mart/:keyword").get(router::search);
    app.listen("0.0.0.0:4000").await?;
    Ok(())
}