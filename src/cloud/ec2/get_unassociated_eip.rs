use anyhow::Context;
use aws_config::SdkConfig;
use aws_sdk_ec2::operation::describe_addresses::DescribeAddressesOutput;
use crate::core::{Finding, FindingId};

/// Get unassociated Elastic IPs for the given account and region.
pub async fn get_unassociated_eip(config: SdkConfig, account :&str, region: &str) -> anyhow::Result<Vec<Finding>> {
    let eip = fetch_unassociated_eip(config).await.with_context(|| "Failed to get EIPs")?;
    let findings = build_unassociated_eip_findings(eip,account,region).await;
    Ok(findings)
}

/// Fetch unassociated Elastic IPs.
async fn fetch_unassociated_eip(config: SdkConfig) -> anyhow::Result<DescribeAddressesOutput> {
    let client = aws_sdk_ec2::Client::new(&config);
    let resp = client.describe_addresses().send().await.with_context(|| "Failed to get EIPs")?;
    Ok(resp)
}

/// Build findings for unassociated Elastic IPs.
async fn build_unassociated_eip_findings(addresses: DescribeAddressesOutput, account: &str, region: &str) -> Vec<Finding> {
    let mut findings: Vec<Finding> = Vec::new();
    for address in addresses.addresses.unwrap() {
        if address.instance_id.is_none() {
            findings.push(Finding::new(
                FindingId::ElasticIp,
                build_eip_arn(
                    account,
                    region,
                    &address.allocation_id.unwrap()
                )
            ));
        }
    }

    findings
}

/// Build an ARN for an Elastic IP.
fn build_eip_arn(account: &str, region: &str, allocation_id: &str) -> String {
    format!("arn:aws:ec2:{}:{}:elastic-ip/{}", region, account, allocation_id)
}