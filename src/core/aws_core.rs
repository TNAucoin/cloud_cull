use anyhow::Context;
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_sdk_sts::operation::assume_role::AssumeRoleOutput;
use aws_sdk_sts as sts;

/// Get the AWS configuration
pub async fn get_config(role: String, config: &SdkConfig) -> anyhow::Result<SdkConfig> {
    let cred_provider = aws_config::sts::AssumeRoleProvider::builder(role)
        .session_name("cloud-clutter-cli")
        .configure(config)
        .build()
        .await;
    
    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(cred_provider)
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