# TODO Git

## Features

Create, Edit, Delete and View your self maintained TODO list on GitHub.

> :notebook: TODO-git uses github-cli to edit the TODO list, which are basically github issues.

## Installation

1. Clone the repository

```bash
git clone https://github.com/UtsavBalar1231/TODO-git.git
```

2. Create your own GitHub repository to store the TODO list.

> Example Repository: https://github.com/UtsavBalara1231/todo-list

3. Copy the repository URL and replace it in the `todo.sh` file.

```bash
- GITHUB_USER_NAME="UtsavBalar1231"
- GITHUB_REPOSIORY_NAME="todo-list"

+ GITHUB_USER_NAME="<Your Github Username>"
+ GITHUB_REPOSIORY_NAME="Your Repository Name"
```

## Usage

```text
create | -c    Creates a new TODO list on github
delete | -d    Delete the latest TODO list from github
edit   | -e    Edit TODO list and push it back
view   | -v    View TODO list from github
help   | -h    show this help message
```

### Create

This command will create a new TODO list issue on GitHub.

```bash
./todo.sh create
```

or

```bash
./todo.sh -c
```

### Delete

This command will delete the latest TODO list issue from GitHub.

```bash
./todo.sh delete
```

or

```bash
./todo.sh -d
```

### Edit

This command will open the TODO list issue in your default editor.
The default editor can be changed by setting the `EDITOR` environment variable.

> :warning: Make sure to save the file before closing the editor.

```bash
./todo.sh edit
```

or

```bash
./todo.sh -e
```

### View

This command will print the TODO list in your terminal emulator window.

```bash
./todo.sh view
```

or

```bash
./todo.sh -v
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
