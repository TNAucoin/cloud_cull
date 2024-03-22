use anyhow::Context;
use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_sts::operation::assume_role::AssumeRoleOutput;
use aws_sdk_sts as sts;

/// Get the AWS configuration
pub async fn get_config() -> anyhow::Result<SdkConfig> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    Ok(config)
}

/// Get credentials for the given role
pub async fn get_credentials(
    config: &SdkConfig,
    account: &str,
    role: &str,
) -> anyhow::Result<AssumeRoleOutput> {
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