use anyhow::Context;
use aws_config::SdkConfig;
use aws_sdk_cloudwatchlogs::Client;
use aws_sdk_cloudwatchlogs::types::LogGroup;
use futures::{stream, StreamExt};

use crate::core::{Finding, FindingId};

pub async fn get_logs_with_no_retention(config: SdkConfig) -> anyhow::Result<Vec<Finding>> {
    // TODO: move this to be an argument
    let temp_prefix = vec![
        String::from("/aws"),
        String::from("test"),
        String::from("/ecs"),
    ];
    let logs = retrieve_log_groups(config, temp_prefix)
        .await
        .with_context(|| "Failed to get logs")?;
    Ok(logs)
}

async fn retrieve_log_groups(
    config: SdkConfig,
    log_group_prefix: Vec<String>,
) -> anyhow::Result<Vec<Finding>> {
    let mut log_group_finding: Vec<Finding> = Vec::new();

    let log_client = aws_sdk_cloudwatchlogs::Client::new(&config);
    let mut stream = stream::iter(
        log_group_prefix
            .into_iter()
            .map(|prefix| fetch_cloudwatch_log_groups(&log_client, prefix)),
    );

    while let Some(result) = stream.next().await {
        let log_groups = result.await;
        match log_groups {
            Ok(groups) => {
                log_group_finding.extend(create_log_group_finding(&groups));
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
    Ok(log_group_finding)
}

fn create_log_group_finding(log_group: &Vec<LogGroup>) -> Vec<Finding> {
    let mut log_group_finding: Vec<Finding> = Vec::new();
    for group in log_group {
        if let Some(log_arn) = &group.arn {
            log_group_finding.push(Finding::new(FindingId::LogGroup, log_arn.to_string()));
        }
    }
    log_group_finding
}

async fn fetch_cloudwatch_log_groups(
    client: &Client,
    prefix: String,
) -> anyhow::Result<Vec<LogGroup>> {
    let mut log_group_results: Vec<LogGroup> = Vec::new();

    let mut log_groups = client
        .describe_log_groups()
        .log_group_name_prefix(prefix)
        .send()
        .await
        .with_context(|| "Failed to describe log groups")?;

    log_group_results.extend(log_groups.log_groups.unwrap_or_default());

    while let Some(next) = log_groups.next_token {
        log_groups = client
            .describe_log_groups()
            .next_token(next)
            .send()
            .await
            .with_context(|| "Failed to describe log groups")?;
        log_group_results.extend(log_groups.log_groups.unwrap_or_default());
    }
    Ok(log_group_results)
}
