#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$(dirname $(realpath $0))"

if [ -n "${SCRIPT_DIR}" ]; then
	source "$SCRIPT_DIR"/../.env
else
	source ../.env
fi

git config --global user.name "$GIT_USER_NAME"
git config --global user.email "$GIT_USER_EMAIL"
git config --global user.signingkey "key::$GIT_USER_SIGNING_KEY"

git config --global advice.detachedHead false
git config --global pull.rebase true
git config --global core.editor "code --wait"

git config --global commit.gpgsign true
git config --global gpg.format ssh
git config --global gpg.ssh.allowedSignersFile "$HOME"/.config/git/allowed_signers

git config --global alias.co "checkout"
git config --global alias.br "branch"
git config --global alias.ci "commit"
git config --global alias.st "status"
git config --global alias.cian "commit --amend --no-edit"