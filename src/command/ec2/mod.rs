use anyhow::Context;
use aws_config::SdkConfig;
use clap::Parser;
use serde::Serialize;

mod eip;
mod volumes;

#[derive(Debug, Serialize, Parser)]
pub struct Ec2 {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Serialize, Parser)]
pub enum Command {
    GetAvailableEc2Volumes(volumes::EbsVolume),
    GetUnassignedEips(eip::Eip),
}

impl Ec2 {
    pub async fn run(&self, config: SdkConfig, account: &str, region: &str) -> anyhow::Result<()> {
        match &self.command {
            Command::GetAvailableEc2Volumes(command) => {
                command.run(config, account, region).await.with_context(|| {
                    format!(
                        "Failed to get available EBS volumes for account {} in region {}",
                        account, region
                    )
                });
                Ok(())
            }
            Command::GetUnassignedEips(command) => {
                command.run(config, account, region).await.with_context(|| {
                    format!(
                        "Failed to get unassociated Elastic IPs for account {} in region {}",
                        account, region
                    )
                });
                Ok(())
            }
        }
    }
}
