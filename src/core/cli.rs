use anyhow::Context;
use clap::{arg, ArgMatches, command, Command};

use crate::core::command_actions;

pub fn gather_matches() -> ArgMatches {
    command!()
        .subcommand(
            Command::new("ebs")
                .about("Elastic Block Store (EBS) commands")
                .subcommand(
                    Command::new("get-available-ebs-volumes")
                        .about("Get available EBS volumes")
                        .arg(arg!(-a --account <ACCOUNT> "account number"))
                        .arg(arg!(-R --role <ROLE> "role name"))
                        .arg(arg!(-r --region <REGION> "region")),
                ),
        )
        .subcommand(
            Command::new("logs")
                .about("CloudWatch Logs commands")
                .subcommand(
                    Command::new("get-log-groups-without-retention")
                        .about("Get log groups without retention")
                        .arg(arg!(-a --account <ACCOUNT> "account number"))
                        .arg(arg!(-R --role <ROLE> "role name"))
                        .arg(arg!(-r --region <REGION> "region")),
                ),
        )
        .get_matches()
}

pub async fn process_matches(matches: &ArgMatches) -> anyhow::Result<(), anyhow::Error> {
    match matches.subcommand() {
        Some(("ebs", sub_m)) => match sub_m.subcommand() {
            Some(("get-available-ebs-volumes", args)) => {
                let role: &String = args.get_one("role").expect("Role is required");
                let account: &String = args.get_one("account").expect("Account is required");
                let region: &String = args.get_one("region").expect("Region is required");
                command_actions::get_available_ebs_volumes(role, account, region)
                    .await
                    .with_context(|| {
                        format!("Failed to get available EBS volumes for role {}", role)
                    })
            }
            _ => handle_invalid_subcommand(),
        },
        Some(("logs", sub_m)) => match sub_m.subcommand() {
            Some(("get-log-groups-without-retention", args)) => {
                let role: &String = args.get_one("role").expect("Role is required");
                let account: &String = args.get_one("account").expect("Account is required");
                let region: &String = args.get_one("region").expect("Region is required");
                command_actions::get_log_groups_without_retention(role, account, region)
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to get log groups without retention for role {}",
                            role
                        )
                    })
            }
            _ => handle_invalid_subcommand(),
        },
        _ => handle_invalid_subcommand(),
    }
}

/// Handle the case where no valid subcommand was found. This is handled by clap.
fn handle_invalid_subcommand() -> anyhow::Result<()> {
    Ok(())
}
