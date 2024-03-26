use clap::{arg, command, ArgMatches, Command};

/// Gather all commands for the CLI.
/// If you need to include extra arguments for a command, you can add them here.
/// For example, if you need to add a `--name` argument to the `ec2` command, you can add it here.
/// ```rust
///   command!()
///      .subcommand(create_command(
///         "ec2",
///        "EC2 commands",
///       &common_args(),
///      Some(&vec![arg!(-n --name <NAME> "name")]),
///    ))
/// ```
/// This will add a `--name` argument to the `ec2` command.
pub fn gather_commands() -> ArgMatches {
    command!()
        .subcommand(
            create_base_command("ec2", "EC2 commands")
                .subcommands(vec![
                    create_sub_command(
                        "get-available-ec2-volumes",
                        "Get available EC2 volumes",
                        &common_args(),
                        None,
                    ),
                    create_sub_command(
                        "get-unassigned-eips",
                        "Get unassociated EIPs",
                        &common_args(),
                        None,
                    ),
                ]),
        )
        .subcommand(
            create_base_command("logs", "CloudWatch Logs commands").subcommand(create_sub_command(
                "get-log-groups-without-retention",
                "Get log groups without retention",
                &common_args(),
                None,
            )),
        )
        .get_matches()
}

/// Common arguments for all commands.
fn common_args() -> Vec<clap::Arg> {
    vec![
        arg!(-a --account <ACCOUNT> "account number").required(true),
        arg!(-R --role <ROLE> "role name").required(true),
        arg!(-r --region <REGION> "region").required(true),
    ]
}

/// Create a new command with the given name and about.
/// This is what you will use to create a new command.
/// Subcommands can be added to this command.
fn create_base_command(name: &str, about: &str) -> Command {
    Command::new(name.to_owned()).about(about.to_owned())
}

/// Create a new command with the given name, about, and arguments.
/// This is what you will use to create a new subcommand.
/// Arguments can be added to this command. And should include the common arguments.
fn create_sub_command(
    name: &str,
    about: &str,
    args: &[clap::Arg],
    other_args: Option<&Vec<clap::Arg>>,
) -> Command {
    // Combine the common arguments with any extra arguments.
    let mut all_args = args.to_owned();
    if let Some(args) = other_args {
        all_args.extend(args.clone());
    }
    // Create the command with the given name, about, and arguments.
    Command::new(name.to_owned())
        .about(about.to_owned())
        .args(&all_args)
}
