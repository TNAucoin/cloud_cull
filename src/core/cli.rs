use clap::{arg, ArgMatches, command, Command};

pub fn main() -> ArgMatches {
    let matches = command!()
        .subcommand(
            Command::new("get-credentials")
                .about("Get credentials for a role")
                .arg(arg!(-a --account <ACCOUNT> "account number"))
                .arg(arg!(-r --role <ROLE> "role name")),
        )
        .get_matches();
    matches
}