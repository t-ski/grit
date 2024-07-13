#!/bin/bash


bash "$(dirname "$0")/_create.sh"

bash "$(dirname "$0")/_run.sh"
if [ $? -eq 0 ]; then
    echo -e "\033[1m\033[32m✓ TESTS SUCCESSFUL\033[0m"
else
    echo -e "\033[1m\033[31m✗ TESTS FAILED\033[0m"
fi

bash "$(dirname "$0")/_delete.sh"