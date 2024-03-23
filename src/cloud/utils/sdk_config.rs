use aws_config::{BehaviorVersion, Region, SdkConfig};

/// Get the default AWS configuration
pub async fn get_default_config(region: &String) -> anyhow::Result<SdkConfig> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region.to_string()))
        .load()
        .await;
    Ok(config)
}

/// Get the AWS configuration with the given role and account
pub async fn get_assume_role_config(
    role: &String,
    account: &String,
    region: &String,
    config: &SdkConfig,
) -> anyhow::Result<SdkConfig> {
    let role_arn = format!("arn:aws:iam::{}:role/{}", account, role);
    let cred_provider = aws_config::sts::AssumeRoleProvider::builder(role_arn)
        .session_name("cloud-clutter-cli")
        .region(Region::new(region.to_string()))
        .configure(config)
        .build()
        .await;

    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(cred_provider)
        .region(Region::new(region.to_string()))
        .load()
        .await;

    Ok(config)
}
