use anyhow::Context;
use aws_config::SdkConfig;

/// Tag the given resources with the tag.
pub async fn tag_with_resource_arns(
    config: SdkConfig,
    resource_arns: &[String],
    key: &str,
    value: &str,
) -> anyhow::Result<()> {
    let tagging_client = aws_sdk_resourcegroupstagging::Client::new(&config);
    tagging_client
        .tag_resources()
        .set_resource_arn_list(Some(resource_arns.iter().map(|x| x.to_string()).collect()))
        .tags(key, value)
        .send()
        .await
        .with_context(|| "Failed to tag EBS volumes")?;
    Ok(())
}
