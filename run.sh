#!/bin/bash

# run.sh v0.2.0
# 2024-10-19

RED='\033[0;31m'
YELLOW='\033[0;33m'
RESET='\033[0m'

# $1 string - question to ask
# Ask a yes no question, only accepts `y` or `n` as a valid answer, returns 0 for yes, 1 for no
ask_yn() {
	while true; do
		printf "\n%b%s? [y/N]:%b " "${GREEN}" "$1" "${RESET}"
		read -r answer
		if [[ "$answer" == "y" ]]; then
			return 0
		elif [[ "$answer" == "n" ]]; then
			return 1
		else
			echo -e "${RED}\nPlease enter 'y' or 'n'${RESET}"
		fi
	done
}

error_close() {
	echo -e "\n${RED}ERROR - EXITED: ${YELLOW}$1${RESET}\n"
	exit 1
}

if ! [ -x "$(command -v dialog)" ]; then
	error_close "dialog is not installed"
fi

production_up() {
	docker compose up -d
}

production_down() {
	docker compose -f docker-compose.yml down
}

production_rebuild() {
	docker compose up -d --build
}

git_pull_branch() {
	git checkout -- .
	git checkout main
	git pull origin main
	git fetch --tags
	latest_tag=$(git tag | sort -V | tail -n 1)
	git checkout -b "$latest_tag"
	sleep 10
}

pull_branch() {
	GIT_CLEAN=$(git status --porcelain)
	if [ -n "$GIT_CLEAN" ]; then
		echo -e "\n${RED}GIT NOT CLEAN${RESET}\n"
		printf "%s\n" "${GIT_CLEAN}"
	fi
	if [[ -n "$GIT_CLEAN" ]]; then
		if ! ask_yn "Happy to clear git state"; then
			exit
		fi
	fi
	git_pull_branch
	main
}

main() {
	cmd=(dialog --backtitle "Start container" --radiolist "choose environment" 14 80 16)
	options=(
		1 "up" off
		2 "down" off
		3 "rebuild" off
		4 "pull & branch" off
	)
	choices=$("${cmd[@]}" "${options[@]}" 2>&1 >/dev/tty)
	exitStatus=$?
	clear
	if [ $exitStatus -ne 0 ]; then
		exit
	fi
	for choice in $choices; do
		case $choice in
		0)
			exit
			;;
		1)
			production_up
			break
			;;
		2)
			production_down
			break
			;;
		3)
			production_rebuild
			break
			;;
		4)
			pull_branch
			;;
		esac
	done
}

main
