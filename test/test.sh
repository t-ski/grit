#!/bin/bash


cargo build --manifest-path="$(dirname "$0")/../Cargo.toml"

run_helper() {
    echo "";
    bash "$(dirname "$0")/_$1.sh"
}

run_helper "repo.create"

run_helper "run"
if [ $? -eq 0 ]; then
    echo -e "\033[1m\033[32m✓ TESTS SUCCESSFUL\033[0m"
else
    echo -e "\033[1m\033[31m✗ TESTS FAILED\033[0m"
fi

run_helper "repo.delete"