#/!bin/bash


VALID_SCRIPT=false

while IFS="=" read -r col1 col2
do
    if [[ $(echo $col1 | xargs) == $1 ]]; then
        eval $col2
        VALID_SCRIPT=true
    fi
done < "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)/scripts.ini"

if [[ $VALID_SCRIPT == false ]]; then
    echo "\033[31mUnknown script\033[0m"
    exit 1
fi