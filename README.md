# todo-git in rust

## Features

A simple GitHub issues based todo list application written in Rust.
Create, Edit, Delete and View your self maintained TODO list on GitHub.

> [!IMPORTANT]
> TODO-git uses github API to modify the TODO list, which are basically github issues.

## Installation

> [!WARNING]
> You must have Rust installed on your computer.
> You can install Rust by running the following command.
> ```bash
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
> ```

1. Clone the repository

```bash
git clone https://github.com/UtsavBalar1231/todo-git-rs.git
```

2. Build the project by running

```bash
cargo build --release
```

3. Copy the ./target/release/todo-git to your local computer.

```bash
cp ./target/release/todo-git /usr/local/bin/todo-git
```

4. Create your own GitHub repository to store the TODO list.

> [!NOTE]
> Example Repository: https://github.com/UtsavBalara1231/todo-list

5. Create the todo-git configuration file. Read how to create the config file
below.

## Usage

```text
A simple GitHub issues based todo list application written in Rust

Usage: todo-git <COMMAND>

Commands:
  edit, -e           Edit the todo-list issue
  view, -v           View the todo-list issue
  delete, -d         Deletes the todo-list issue
  create-config, -c  Creates a local todo-list config
                         Extra option:
                             -i    Create the todo-git config interactively
  new, -n            Creates a new todo-list issue
                         Extra options:
                             -t    Enter title of the new issue
                             -b    Enter body file path for the new issue
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Create configuration file

This command will create a new TODO git configuration file on your local
computer.

It will store information such as your GitHub `username`, `repository name` and
`access token`.

```bash
todo-git create-config
```

or

```bash
todo-git -c
```

Open the configuration file using your editor and fill in the information.

> [!NOTE]
> You can also create the configuration file interactively by running the
> `todo-git create-config interactive` command.
> or
> `todo-git -c -i`

### Delete

This command will delete the latest TODO list issue from GitHub repository.

```bash
todo-git delete
```

or

```bash
todo-git -d
```

### Edit

This command will open the TODO list issue in your default editor.
The default editor can be changed by setting the `EDITOR` environment variable.

> [!NOTE]
> If `EDITOR` is not set in your environment, **vim** will be used as a
fallback editor.

> [!warning]
> Make sure to save the file before closing the editor.

```bash
todo-git edit
```

or

```bash
todo-git -e
```

### View

This command will print the TODO list in your terminal emulator window.

```bash
todo-git view
```

or

```bash
todo-git -v
```

## License

[MIT](./LICENSE)
