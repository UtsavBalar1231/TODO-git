#!/usr/bin/env bash
# shellcheck source=/dev/null
#
# SPDX-License-Identifier: MIT
# Copyright (c) 2023 Utsav Balar (utsavbalar1231@gmail.com)
# Version: 1.0

# This script is used to edit the todo list.
# the todo list is hosted on github.
# Use curl to get the todo list from github.
# Use your default EDITOR to edit the todo list.
# Use curl to push the todo list to github.
# Use github-cli for all the github operations.

GITHUB_USER_NAME="UtsavBalar1231"
GITHUB_REPOSIORY_NAME="todo-list"

# Check if gh is installed.
check_gh_installation() {
	if ! command -v gh &>/dev/null; then
		echo "Install github-cli to use this script."
		exit 1
	fi
}

# Check if the user is logged in to github.
check_gh_auth_status() {
	# Check if gh is installed.
	check_gh_installation

	if ! gh auth status &>/dev/null; then
		echo "Please login to github using github-cli."
		echo "Use the command 'gh auth login' to login."
		exit 1
	fi
}

# Get the latest issue count.
get_latest_issue_count() {
	check_gh_auth_status

	# Get the latest issue count.
	gh issue list -R "${GITHUB_USER_NAME}/${GITHUB_REPOSIORY_NAME}" | sed -n '1p' | awk '{print $1}'
}

# Get the git editor.
get_git_editor() {
	if [ -z "${EDITOR}" ]; then
		if command -v nvim &>/dev/null; then
			echo "nvim"
		elif command -v vim &>/dev/null; then
			echo "vim"
		else
			echo "Please install vim or nvim to use this script."
			exit 1
		fi
	else
		echo "${EDITOR}"
	fi
}

GITHUB_ISSUE_NUMBER=$(get_latest_issue_count)
GIT_EDITOR=$(get_git_editor)

# Edit the todo list.
edit_todo() {
	check_gh_auth_status

	if [ -f /tmp/todo.md ]; then
		rm /tmp/todo.md
	fi

	# Parse and save the todo list to temp file.
	gh issue view -R "${GITHUB_USER_NAME}/${GITHUB_REPOSIORY_NAME}" "${GITHUB_ISSUE_NUMBER}" | sed -n '11,$p' >/tmp/todo.md

	${GIT_EDITOR} /tmp/todo.md

	# Push the todo list to github.
	if [ -f /tmp/todo.md ]; then
		gh issue edit -R "${GITHUB_USER_NAME}/${GITHUB_REPOSIORY_NAME}" "${GITHUB_ISSUE_NUMBER}" -F /tmp/todo.md
	else
		echo "Error occured while making changes made to the todo list."
		exit 1
	fi
}

view_todo() {
	check_gh_auth_status

	if ! gh issue view -R "${GITHUB_USER_NAME}/${GITHUB_REPOSIORY_NAME}" "${GITHUB_ISSUE_NUMBER}"; then
		echo "Error occured while fetching the todo list."
		exit 1
	fi
}

create_todo() {
	check_gh_auth_status

	if ! gh issue create -R "${GITHUB_USER_NAME}/${GITHUB_REPOSIORY_NAME}" -t "TODO List" -b "TODO List"; then
		echo "Error occured while creating the todo list."
		exit 1
	fi
}

delete_todo() {
	check_gh_auth_status

	if ! gh issue delete -R "${GITHUB_USER_NAME}/${GITHUB_REPOSIORY_NAME}" "${GITHUB_ISSUE_NUMBER}"; then
		echo "Error occured while deleting the todo list."
		exit 1
	fi
}

usage() {
	local usage
	usage=$(
		cat <<EOF
${0} is a tool to edit the TODO list on github.
${0} uses github-cli to edit the TODO list, which are basically github issues.

Usage: ${0} [OPTIONS]"

Options:
	create | -c    Creates a new TODO list on github
	delete | -d    Delete the latest TODO list from github
	edit   | -e    Edit TODO list and push it back
	view   | -v    View TODO list from github
	help   | -h    show this help message

EOF
	)

	echo "${usage}"
}

OPTIONS=("${@:-help}")
for option in "${OPTIONS[@]}"; do
	case ${option} in
	create | -c)
		create_todo
		;;
	delete | -d)
		delete_todo
		;;
	edit | -e)
		edit_todo
		;;
	help | -h)
		usage
		;;
	view | -v)
		view_todo
		;;
	*)
		echo "unknown option: $option"
		usage
		;;
	esac
done
