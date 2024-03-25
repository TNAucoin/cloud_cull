use crate::cloud::ebs;
use crate::cloud::log;
use crate::cloud::utils;

/// Get all available EBS volumes for the given account and region.
pub async fn get_available_ebs_volumes(
    role: &str,
    account: &str,
    region: &str,
) -> anyhow::Result<()> {
    let config = utils::get_assume_role_config_with_defaults(role, account, region).await?;
    let available_volumes = ebs::get_available_ebs_volumes(config, region, account).await?;
    println!("Available EBS volumes: {:?}", available_volumes);
    Ok(())
}

/// Get log groups without retention for the given account and region.
pub async fn get_log_groups_without_retention(
    role: &str,
    account: &str,
    region: &str,
) -> anyhow::Result<()> {
    let config = utils::get_assume_role_config_with_defaults(role, account, region).await?;
    let _logs = log::get_logs_with_no_retention(config).await?;
    Ok(())
}
