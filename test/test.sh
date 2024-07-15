#!/bin/bash


SOURCE_DIR_PATH=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

test_dir() {
    echo $SOURCE_DIR_PATH
}

run_helper() {
    cd $(test_dir)
    bash "$(test_dir)/_$1.sh"
}

close() {
    echo "";
    if [ $RUN_STATUS -eq 0 ]; then
        echo -e "\033[1m\033[32m✓ TESTS SUCCESSFUL\033[0m"
    else
        echo -e "\033[1m\033[31m✗ TESTS FAILED\033[0m"
    fi
    exit $RUN_STATUS
}

trap close EXIT

echo -e "\033[2m\033[1mBUILD\033[0m"
cargo build --manifest-path="$(test_dir)/../Cargo.toml"
if [ ! $? -eq 0 ]; then
    exit 2
fi

echo "";
run_helper "repo.create"

run_helper "run"
RUN_STATUS=$?

echo "";
run_helper "repo.delete"