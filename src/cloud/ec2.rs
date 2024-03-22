use anyhow::{Context, Result};
use aws_sdk_ec2::types::Filter;

pub async fn get_available_ebs_volumes(config: aws_config::SdkConfig) -> Result<Vec<String>> {
    // create a new struct to hold volume findings
    let mut volume_ids: Vec<String> = Vec::new();
    let client = aws_sdk_ec2::Client::new(&config);
    let resp = client
        .describe_volumes()
        .filters(Filter::builder().name("status").values("available").build())
        .send()
        .await
        .with_context(|| "Failed to get available EBS volumes")?;
    let volumes = resp.volumes.unwrap_or_default();
    //return volume ids
    for volume in volumes {
        if let Some(id) = volume.volume_id {
            volume_ids.push(id);
        }
    }
    Ok(volume_ids)
}
