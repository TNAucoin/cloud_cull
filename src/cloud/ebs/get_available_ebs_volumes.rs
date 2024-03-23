use anyhow::{Context, Result};
use aws_config::SdkConfig;
use aws_sdk_ec2::types::Filter;

pub async fn get_available_ebs_volumes(
    config: SdkConfig,
    region: &String,
    account: &String,
) -> Result<Vec<String>> {
    // create a new structs to hold volume findings
    let mut volume_ids: Vec<String> = Vec::new();
    let mut volume_arns: Vec<String> = Vec::new();

    let tagging_client = aws_sdk_resourcegroupstagging::Client::new(&config);
    // get all volumes
    let mut volume_response = call_describe_volumes(&config, String::from("")).await?;
    volume_ids.append(volume_response.volumes.as_mut());

    while let Some(token) = volume_response.next_token.clone() {
        println!("token: {}", token);
        volume_response = call_describe_volumes(&config, token).await?;
        volume_ids.append(volume_response.volumes.as_mut());
    }
    volume_arns.extend(
        create_ebs_volume_arns(&volume_ids, &account, &region)
            .iter()
            .cloned()
            .map(|x| x)
            .collect::<Vec<String>>(),
    );

    // TODO: Move this out of this function
    let tag_resp = tagging_client
        .tag_resources()
        .set_resource_arn_list(Some(volume_arns.iter().map(|x| x.to_string()).collect()))
        .tags("cloud_clutter", "true")
        .send()
        .await
        .with_context(|| "Failed to tag EBS volumes")?;
    println!("{:?}", tag_resp);

    Ok(volume_arns)
}

fn create_ebs_volume_arns(
    volume_ids: &Vec<String>,
    account: &String,
    region: &String,
) -> Vec<String> {
    volume_ids
        .iter()
        .map(|id| format!("arn:aws:ec2:{region}:{account}:volume/{id}"))
        .collect()
}

// Response struct for describe_volumes
struct VolumeResponse {
    volumes: Vec<String>,
    next_token: Option<String>,
}

async fn call_describe_volumes(config: &SdkConfig, token: String) -> Result<VolumeResponse> {
    // TODO: we should skip volumes that are already tagged, this is a bit tricky
    let client = aws_sdk_ec2::Client::new(&config);
    let resp = client
        .describe_volumes()
        .max_results(100)
        .filters(Filter::builder().name("status").values("available").build())
        .next_token(token)
        .send()
        .await
        .with_context(|| "Failed to get available EBS volumes")?;

    let volumes: Vec<String> = resp
        .volumes
        .unwrap()
        .iter()
        .filter_map(|x| x.volume_id.as_ref())
        .map(|x| x.to_string())
        .collect();

    Ok(VolumeResponse {
        volumes,
        next_token: resp.next_token,
    })
}
