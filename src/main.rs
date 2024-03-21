use anyhow::{Context, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_sts as sts;
use aws_sdk_sts::operation::assume_role::AssumeRoleOutput;
use clap::{arg, command, ArgMatches, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli_main();
    let config = get_config().await?;
    if let Some(matches) = matches.subcommand_matches("get-credentials") {
        if let Some(account) = matches.get_one::<String>("account") {
            if let Some(role) = matches.get_one::<String>("role") {
                let role = get_credentials(&config, &account, &role).await?;
                println!("{:#?}", role.credentials.unwrap());
            }
        }
    }
    Ok(())
}
/// Get the AWS configuration
async fn get_config() -> Result<SdkConfig> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    Ok(config)
}

/// Get credentials for the given role
async fn get_credentials(
    config: &SdkConfig,
    account: &str,
    role: &str,
) -> Result<AssumeRoleOutput> {
    let client = sts::Client::new(config);
    let role_arn = format!("arn:aws:iam::{}:role/{}", account, role);
    let role = client
        .assume_role()
        .role_arn(&role_arn)
        .role_session_name("clutter_cull_cli")
        .send()
        .await
        .with_context(|| format!("could not assume role {}", &role_arn))?;

    Ok(role)
}

fn cli_main() -> ArgMatches {
    let matches = command!()
        .subcommand(
            Command::new("get-credentials")
                .about("Get credentials for a role")
                .arg(arg!(-a --account <ACCOUNT> "account number"))
                .arg(arg!(-r --role <ROLE> "role name")),
        )
        .get_matches();
    matches
}
