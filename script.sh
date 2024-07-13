#/!bin/bash


while IFS="=" read -r col1 col2
do
    if [[ $(echo $col1 | xargs) == $1 ]]; then
        eval $(echo $col2 | xargs)
        exit 0
    fi
done < "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)/scripts.ini"

echo "\033[31mUnknown script\033[0m"
exit 1