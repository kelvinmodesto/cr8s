use clap::{Arg, Command};

extern crate c8rs;

fn main() {
    let matches = Command::new("c8rs")
        .about("c8rs commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("user management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("create new user")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("role")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new("list").about("list existing users"))
                .subcommand(
                    Command::new("delete")
                        .about("delete user by ID")
                        .arg_required_else_help(true)
                        .arg(Arg::new("id").required(true)),
                ),
        )
        .get_matches();

    if let Some(("users", sub_matches)) = matches.subcommand() {
        match sub_matches.subcommand() {
            Some(("create", create_sub_matches)) => {}
            Some(("list", list_sub_matches)) => {}
            Some(("delete", delete_sub_matches)) => {}
            _ => {}
        }
    }
}
