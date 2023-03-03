#!/bin/zsh

set -e
source scripts/style.sh

CONFIRMATION_WORD="prod"

print -n "Type $(style red bold underline $CONFIRMATION_WORD) to confirm push to remote $(style blue underline main) branch:\n\t $(style-open white)> "
set-style white bold underline
read CONFIRMATION
reset-style

if [[ $CONFIRMATION == $CONFIRMATION_WORD ]]; then
    style green bold underline "Confirmed"
    exit 0
else
    style yellow bold underline "Cancelled"
    exit 69
fi