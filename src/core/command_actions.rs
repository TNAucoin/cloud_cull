use aws_config::BehaviorVersion;

use crate::cloud::ebs;

/// Get all available EBS volumes for the given account and region.
pub async fn get_available_ebs_volumes(
    role: &String,
    account: &String,
    region: &String,
) -> anyhow::Result<()> {
    let local_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = crate::core::get_config(role, account, region, &local_config).await?;
    let available_volumes = ebs::get_available_ebs_volumes(config, region, account).await?;
    println!("{:?}", available_volumes);
    Ok(())
}
