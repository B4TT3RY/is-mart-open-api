use async_std::task;
use eyre::Result;

fn main() -> Result<()> {
    task::block_on(start())?;
    Ok(())
}

async fn start() -> Result<()> {
    Ok(())
}