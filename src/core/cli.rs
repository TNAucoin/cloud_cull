use anyhow::Context;
use clap::ArgMatches;

use crate::core::command_actions;

pub async fn process_matches(matches: &ArgMatches) -> anyhow::Result<(), anyhow::Error> {
    if let Some((command, sub_m)) = matches.subcommand() {
        if let Some((subcommand, args)) = sub_m.subcommand() {
            // These arguments are required for all subcommands
            let role: &String = args.get_one("role").expect("Role is required");
            let account: &String = args.get_one("account").expect("Account is required");
            let region: &String = args.get_one("region").expect("Region is required");

            match (command, subcommand) {
                ("ec2", "get-available-ec2-volumes") => {
                    command_actions::get_available_ebs_volumes(role, account, region)
                        .await
                        .with_context(|| {
                            format!("Failed to get available EBS volumes for role {}", role)
                        })
                }
                ("ec2", "get-unassigned-eips") => {
                    command_actions::get_unassociated_eip(role, account, region)
                        .await
                        .with_context(|| {
                            format!("Failed to get unassociated EIPs for role {}", role)
                        })
                }
                ("logs", "get-log-groups-without-retention") => {
                    command_actions::get_log_groups_without_retention(role, account, region)
                        .await
                        .with_context(|| {
                            format!(
                                "Failed to get log groups without retention for role {}",
                                role
                            )
                        })
                }
                // Catch all for invalid command and subcommand combinations
                _ => handle_invalid_subcommand(),
            }
        } else {
            handle_invalid_subcommand()
        }
    } else {
        handle_invalid_subcommand()
    }
}

/// Handle the case where no valid subcommand was found. This is handled by clap.
fn handle_invalid_subcommand() -> anyhow::Result<()> {
    Ok(())
}
