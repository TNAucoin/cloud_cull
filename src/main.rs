use anyhow::Result;

use cloud_cull::core;

#[tokio::main]
async fn main() -> Result<()> {
    core::Eco::run_from_args().await?;
    // let matches = core::gather_commands();
    // core::process_matches(&matches).await?;
    Ok(())
}
