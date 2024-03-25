use crate::cloud::ebs;
use crate::cloud::utils;

/// Get all available EBS volumes for the given account and region.
pub async fn get_available_ebs_volumes(
    role: &str,
    account: &str,
    region: &str,
) -> anyhow::Result<()> {
    let config = utils::get_assume_role_config_with_defaults(role, account, region).await?;
    let available_volumes = ebs::get_available_ebs_volumes(config, region, account).await?;
    println!("{:?}", available_volumes);
    Ok(())
}
