use std::env;

pub enum TodoGitCommand {
    Edit,
    View,
    Delete,
    CreateConfig(bool),
    New {
        title: Option<String>,
        body: Option<String>,
    },
    Help,
}

impl TodoGitCommand {
    fn usage() {
        println!("Usage: {} <command>", env!("CARGO_PKG_NAME"));
        println!("Commands:");
        println!("  edit (-e)                - Edit the todo-list issue");
        println!("  view (-v)                - View the todo-list issue");
        println!("  delete (-d)              - Deletes the todo-list issue");
        println!("  create-config (-c)       - Creates a local todo-list config");
        println!("      -i, --interactive    - Create the todo-git config interactively");
        println!("  new (-n)                 - Creates a new todo-list issue");
        println!("      -t, --title <title>  - Enter title of the new issue");
        println!("      -b, --body <body>    - Enter body file path for the new issue");
        println!("  help                     - Show this help message");
    }

    pub fn parse() -> Option<Self> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            Self::usage();
            return None;
        }

        let subcommand = match args[1].as_str() {
            "edit" | "-e" | "e" => Some(TodoGitCommand::Edit),
            "view" | "-v" | "v" => Some(TodoGitCommand::View),
            "delete" | "-d" | "d" => Some(TodoGitCommand::Delete),
            "create-config" | "-c" => {
                let interactive = args.iter().any(|arg| arg == "-i" || arg == "--interactive");
                Some(TodoGitCommand::CreateConfig(interactive))
            }
            "new" | "-n" => {
                let mut title = None;
                let mut body = None;
                let mut i = 2;

                while i < args.len() {
                    match args[i].as_str() {
                        "-t" | "--title" => {
                            i += 1;
                            if i < args.len() {
                                title = Some(args[i].clone());
                            }
                        }
                        "-b" | "--body" => {
                            i += 1;
                            if i < args.len() {
                                body = Some(args[i].clone());
                            }
                        }
                        _ => {
                            println!("Invalid option: {}", args[i]);
                            println!("       todo-git new -t <title> -b <body>");
                            println!("-t and -b are required");
                            return None;
                        }
                    }
                    i += 1;
                }

                Some(TodoGitCommand::New { title, body })
            }
            "help" | "-h" | "h" => {
                Self::usage();
                Some(TodoGitCommand::Help)
            }
            _ => {
                println!("Invalid command: {}", args[1]);
                Self::usage();
                None
            }
        };

        subcommand
    }
}
