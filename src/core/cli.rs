use clap::{arg, command, ArgMatches, Command};

pub fn main() -> ArgMatches {
    let matches = command!()
        .subcommand(
            Command::new("ec2").subcommand(
                Command::new("get-available-ebs-volumes")
                    .about("Get available EBS volumes")
                    .arg(arg!(-a --account <ACCOUNT> "account number"))
                    .arg(arg!(-r --role <ROLE> "role name")),
            ),
        )
        .get_matches();
    matches
}
