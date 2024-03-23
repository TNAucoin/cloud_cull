use crate::cloud::ebs;
use crate::cloud::utils;

/// Get all available EBS volumes for the given account and region.
pub async fn get_available_ebs_volumes(
    role: &String,
    account: &String,
    region: &String,
) -> anyhow::Result<()> {
    let local_config = utils::get_default_config(region).await?;
    let config = utils::get_assume_role_config(role, account, region, &local_config).await?;
    let available_volumes = ebs::get_available_ebs_volumes(config, region, account).await?;
    println!("{:?}", available_volumes);
    Ok(())
}
