use anyhow::{Context, Result};
use aws_config::SdkConfig;
use aws_sdk_ec2::types::Filter;

use crate::core::{Finding, FindingId};

//TODO: move this to be an argument
const MAX_RESULTS: i32 = 100;

// Response struct for describe_volumes
struct VolumeResponse {
    volumes: Vec<String>,
    next_token: Option<String>,
}

/// Get all available EBS volumes for the given account and region.
pub async fn get_available_ebs_volumes(
    config: SdkConfig,
    region: &str,
    account: &str,
) -> Result<Vec<Finding>> {
    // create a new structs to hold volume findings
    let mut volume_findings: Vec<Finding> = Vec::new();

    // get all available volumes
    let volume_ids = fetch_available_volume_ids(&config, MAX_RESULTS)
        .await
        .with_context(|| "Failed to get available EBS volumes")?;

    volume_findings.extend(create_ebs_volume_findings(&volume_ids, account, region).to_vec());
    // TODO: tag the volumes

    Ok(volume_findings)
}

/// Create the EBS volume ARNs for the given volume IDs.
fn create_ebs_volume_findings(volume_ids: &[String], account: &str, region: &str) -> Vec<Finding> {
    volume_ids
        .iter()
        .map(|id| {
            Finding::new(
                FindingId::EbsVolume,
                format!("arn:aws:ec2:{}:{}:volume/{}", region, account, id),
            )
        })
        .collect()
}

/// Fetch all available EBS volumes.
async fn fetch_available_volume_ids(config: &SdkConfig, max_results: i32) -> Result<Vec<String>> {
    let mut volume_ids: Vec<String> = Vec::new();
    let mut token = String::from("");
    loop {
        let volume_response = fetch_ec2_volume_details(config, token.clone(), max_results).await?;
        volume_ids.extend(volume_response.volumes);

        if let Some(next_token) = volume_response.next_token {
            token = next_token;
        } else {
            break;
        }
    }

    Ok(volume_ids)
}

/// Call the EC2 describe_volumes API.
async fn fetch_ec2_volume_details(
    config: &SdkConfig,
    token: String,
    max_results: i32,
) -> Result<VolumeResponse> {
    // TODO: we should skip volumes that are already tagged, this is a bit tricky
    let client = aws_sdk_ec2::Client::new(config);
    let resp = client
        .describe_volumes()
        .max_results(max_results)
        .filters(Filter::builder().name("status").values("available").build())
        .next_token(token)
        .send()
        .await
        .with_context(|| "Failed to get available EBS volumes")?;

    let volumes: Vec<String> = resp
        .volumes
        .expect("Volumes not found")
        .iter()
        .filter_map(|x| x.volume_id.as_ref())
        .map(|x| x.to_string())
        .collect();

    Ok(VolumeResponse {
        volumes,
        next_token: resp.next_token,
    })
}
