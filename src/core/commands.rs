use clap::{arg, ArgMatches, command, Command};

/// Gather all commands for the CLI.
pub fn gather_commands() -> ArgMatches {
    command!()
        .subcommand(ec2_commands())
        .subcommand(log_commands())
        .get_matches()
}

/// Wire up the EC2 commands.
fn ec2_commands() -> Command {
    Command::new("ec2").about("EC2 commands").subcommand(
        Command::new("get-available-ec2-volumes")
            .about("Get available EBS volumes")
            .arg(arg!(-a --account <ACCOUNT> "account number"))
            .arg(arg!(-R --role <ROLE> "role name"))
            .arg(arg!(-r --region <REGION> "region")),
    )
}

/// Wire up the CloudWatch Logs commands.
fn log_commands() -> Command {
    Command::new("logs")
        .about("CloudWatch Logs commands")
        .subcommand(
            Command::new("get-log-groups-without-retention")
                .about("Get log groups without retention")
                .arg(arg!(-a --account <ACCOUNT> "account number"))
                .arg(arg!(-R --role <ROLE> "role name"))
                .arg(arg!(-r --region <REGION> "region")),
        )
}
