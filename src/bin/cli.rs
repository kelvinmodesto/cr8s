use clap::{Arg, Command, value_parser};

extern crate c8rs;

#[tokio::main]
async fn main() {
    let matches = Command::new("c8rs")
        .about("c8rs commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("user management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("create a new user")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("roles")
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
                        .arg(
                            Arg::new("id")
                                .required(true)
                                .value_parser(value_parser!(i32)),
                        ),
                )
                .subcommand(
                    Command::new("digest")
                        .about("Send a digest with latest crates via email")
                        .arg(Arg::new("email"))
                        .arg(
                            Arg::new("hours_since")
                                .required(true)
                                .value_parser(value_parser!(i32)),
                        ),
                ),
        )
        .get_matches();

    if let Some(("users", sub_matches)) = matches.subcommand() {
        match sub_matches.subcommand() {
            Some(("create", create_sub_matches)) => {
                c8rs::utils::commands::create_user(
                    create_sub_matches
                        .get_one::<String>("username")
                        .unwrap()
                        .to_owned(),
                    create_sub_matches
                        .get_one::<String>("password")
                        .unwrap()
                        .to_owned(),
                    create_sub_matches
                        .get_many::<String>("roles")
                        .unwrap()
                        .map(|v| v.to_owned())
                        .collect(),
                )
                .await
            }
            Some(("list", _)) => c8rs::utils::commands::list_users().await,
            Some(("delete", delete_sub_matches)) => {
                c8rs::utils::commands::delete_user(
                    delete_sub_matches.get_one::<i32>("id").unwrap().to_owned(),
                )
                .await
            }
            Some(("digest", sub_matches)) => {
                c8rs::utils::commands::digest_send(
                    sub_matches.get_one::<String>("email").unwrap().to_owned(),
                    sub_matches
                        .get_one::<i32>("hours_since")
                        .unwrap()
                        .to_owned(),
                )
                .await
            }
            _ => {}
        }
    }
}
