use anyhow::Result;

use cloud_cull::core;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = core::gather_commands();
    core::process_matches(&matches).await?;
    Ok(())
}
