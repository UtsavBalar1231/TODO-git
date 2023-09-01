use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn cli() -> ArgMatches {
    Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            Command::new("edit")
                .short_flag('e')
                .about("Edit the todo-list issue")
                .args_conflicts_with_subcommands(true),
        )
        .subcommand(
            Command::new("view")
                .short_flag('v')
                .about("View the todo-list issue")
                .args_conflicts_with_subcommands(true),
        )
        .subcommand(
            Command::new("delete")
                .short_flag('d')
                .about("Deletes the todo-list issue")
                .args_conflicts_with_subcommands(true),
        )
        .subcommand(
            Command::new("create-config")
                .short_flag('c')
                .about("Creates a local todo-list config\nExtra option:\n    -i    Create the todo-git config interactively")
                .args_conflicts_with_subcommands(true)
                .arg_required_else_help(false)
                .arg(
                    Arg::new("interactive")
                        .long("interactive")
                        .short('i')
                        .help("Create the todo-git config interactively")
                        .required(false)
                        .action(ArgAction::SetTrue)
                ),
        )
        .subcommand(
            Command::new("new")
                .short_flag('n')
                .about("Creates a new todo-list issue\nExtra options:\n    -t    Enter title of the new issue\n    -b    Enter body file path for the new issue")
                .args_conflicts_with_subcommands(true)
                .arg_required_else_help(false)
                .arg(
                    Arg::new("title")
                        .long("title")
                        .short('t')
                        .help("Enter title of the new issue")
                        .required(false)
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("body")
                        .long("body")
                        .short('b')
                        .help("Enter body file path for the new issue")
                        .required(false)
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .get_matches()
}
