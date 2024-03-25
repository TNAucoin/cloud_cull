use anyhow::Context;
use aws_config::SdkConfig;
use aws_sdk_cloudwatchlogs::Client;
use aws_sdk_cloudwatchlogs::types::LogGroup;
use futures::{stream, StreamExt};

pub async fn get_logs_with_no_retention(config: SdkConfig) -> anyhow::Result<()> {
    let temp_prefix = vec![
        String::from("/aws"),
        String::from("test"),
        String::from("/ecs"),
    ];
    let _logs = get_logs(config, temp_prefix)
        .await
        .with_context(|| "Failed to get logs")?;
    Ok(())
}

async fn get_logs(config: SdkConfig, log_group_prefix: Vec<String>) -> anyhow::Result<()> {
    let log_client = aws_sdk_cloudwatchlogs::Client::new(&config);
    let mut stream = stream::iter(
        log_group_prefix
            .into_iter()
            .map(|prefix| process_log_group_prefix(&log_client, prefix)),
    );

    while let Some(result) = stream.next().await {
        result;
    }
    Ok(())
}

async fn process_log_group_prefix(
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
    println!("Log groups: {:?}", log_group_results);
    Ok(log_group_results)
}
