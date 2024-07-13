#!/bin/bash


run_helper() {
    echo "";
    bash "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)/_$1.sh"
}

run_helper "repo.create"

run_helper "run"
RUN_RESULT=$?

run_helper "repo.delete"

if [ $RUN_RESULT -eq 0 ]; then
    echo -e "\033[1m\033[32m✓ TESTS SUCCESSFUL\033[0m"
else
    echo -e "\033[1m\033[31m✗ TESTS FAILED\033[0m"
    exit 1
fi