use crate::cloud::ec2;
use crate::cloud::log;
use crate::cloud::utils;

/// Get all available EBS volumes for the given account and region.
pub async fn get_available_ebs_volumes(
    role: &str,
    account: &str,
    region: &str,
) -> anyhow::Result<()> {
    let config = utils::get_assume_role_config_with_defaults(role, account, region).await?;
    let volume_findings = ec2::get_available_ebs_volumes(config, region, account).await?;
    println!("Available EBS volumes: {:?}", volume_findings);
    Ok(())
}

/// Get log groups without retention for the given account and region.
pub async fn get_log_groups_without_retention(
    role: &str,
    account: &str,
    region: &str,
) -> anyhow::Result<()> {
    let config = utils::get_assume_role_config_with_defaults(role, account, region).await?;
    let log_findings = log::get_logs_with_no_retention(config).await?;
    println!("Log groups without retention: {:?}", log_findings);
    Ok(())
}

/// Get unassociated EIPs for the given account and region.
pub async fn get_unassociated_eip(
    role: &str,
    account: &str,
    region: &str,
) -> anyhow::Result<()> {
    let config = utils::get_assume_role_config_with_defaults(role, account, region).await?;
    let eip_findings = ec2::get_unassociated_eip(config,account,region).await?;
    println!("Unassociated EIPs: {:?}", eip_findings);
    Ok(())
}
