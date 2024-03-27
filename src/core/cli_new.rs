use anyhow::Context;
use aws_config::SdkConfig;
use clap::Parser;
use serde::Serialize;

use crate::cloud;
use crate::command::{self};

#[derive(Debug, Serialize, Parser)]
#[command(
    name = "Eco",
    about = "cleanup of wasted resources within your AWS environment"
)]
#[command(next_line_help = true)]
pub struct Eco {
    #[clap(subcommand)]
    command: Command,
    #[arg(short, long, global = true, required = false)]
    account: String,
    #[arg(short, long, short = 'R', global = true, required = false)]
    role: String,
    #[arg(short, long, global = true, required = false)]
    region: String,
}
impl Eco {
    pub async fn run_from_args() -> anyhow::Result<()> {
        Eco::parse().run().await?;
        Ok(())
    }
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = self.get_assume_role_config().await.with_context(|| {
            format!(
                "Failed to get assume role config for account {} and role {}",
                self.account, self.role
            )
        })?;
        self.execute_command(config).await?;
        Ok(())
    }

    pub async fn get_assume_role_config(&self) -> anyhow::Result<SdkConfig> {
        cloud::utils::get_assume_role_config_with_defaults(&self.role, &self.account, &self.region)
            .await
    }

    pub async fn execute_command(&self, config: SdkConfig) -> anyhow::Result<()> {
        match &self.command {
            Command::Ec2(command) => command
                .run(config, self.account.as_ref(), self.region.as_ref())
                .await
                .with_context(|| {
                    format!(
                        "Failed to run command {:?} for account {} in region {}",
                        command, self.account, self.region
                    )
                }),
        }
    }
}

#[derive(Debug, Serialize, Parser)]
pub enum Command {
    Ec2(command::Ec2),
}
