use anyhow::{Result};

use crate::core::aws_core;
use crate::core::cli;

mod core;


#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::main();
    let config = aws_core::get_config().await?;
    if let Some(matches) = matches.subcommand_matches("get-credentials") {
        if let Some(account) = matches.get_one::<String>("account") {
            if let Some(role) = matches.get_one::<String>("role") {
                let role = aws_core::get_credentials(&config, &account, &role).await?;
                println!("{:#?}", role.credentials.unwrap());
            }
        }
    }
    Ok(())
}





