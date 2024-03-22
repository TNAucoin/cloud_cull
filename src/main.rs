use anyhow::Result;
use aws_sdk_ebs::config::BehaviorVersion;

use crate::cloud::ec2;
use crate::core::aws_core;
use crate::core::cli;

mod cloud;
mod core;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::main();
    match matches.subcommand() {
        Some(("ec2", sub_m)) => match sub_m.subcommand() {
            Some(("get-available-ebs-volumes", args)) => {
                let role: &String = args.get_one("role").unwrap();
                let account: &String = args.get_one("account").unwrap();
                get_available_ebs_volumes(role,account).await?;
            }
            _ => {
                // No valid subcommand was found, this is handled by clap.
            }
        },
        _ => {}
    }
    Ok(())
}

async fn get_available_ebs_volumes(role: &String, account: &String) -> Result<()> {
    let local_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = aws_core::get_config(role, account, &local_config).await?;
    let available_volumes = ec2::get_available_ebs_volumes(config).await?;
    println!("{:?}", available_volumes);
    Ok(())
}
