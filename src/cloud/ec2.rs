use anyhow::{Result,Context};
use aws_config::imds::credentials::ImdsCredentialsProvider;
use aws_config::provider_config::ProviderConfig;
use aws_sdk_ec2::config::SharedCredentialsProvider;
use aws_sdk_ec2::types::Filter;
use aws_sdk_sts::config::BehaviorVersion;
use aws_sdk_sts::operation::assume_role::AssumeRoleOutput;

pub async fn get_available_ebs_volumes(config : aws_config::SdkConfig, role: AssumeRoleOutput) -> Result<()> {
   let client = aws_config::SdkConfig::builder()
       .credentials_provider(SharedCredentialsProvider::new(ProviderConfig::));
    let resp = client.describe_volumes()
        .filters(Filter::builder().name("status").values("available").build())
        .send()
        .await
        .with_context(|| "Failed to get available EBS volumes")?;
    println!("{:#?}", resp);
    Ok(())
}