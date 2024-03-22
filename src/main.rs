use anyhow::{Result};

use crate::core::cli;
use crate::core::aws_core;
use crate::cloud::ec2;

mod core;
mod cloud;


#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::main();
    let config = aws_core::get_config().await?;
    if let Some(matches) = matches.subcommand_matches("get-credentials") {
        if let Some(account) = matches.get_one::<String>("account") {
            if let Some(role) = matches.get_one::<String>("role") {
                let role = aws_core::get_credentials(&config, &account, &role).await?;
                ec2::get_available_ebs_volumes() 
                println!("{:#?}", role.credentials.unwrap());
            }
        }
    }
    Ok(())
}





