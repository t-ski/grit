source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)/__dirname.sh"

ASSERTIONS=0

grit() {
    $(dirname)/../target/debug/grit $@
}

assert() {
    if [[ $1 == $2 ]]; then
        return
    fi

    echo -e "\033[0m\n\033[31mInaccurate assertion:\033[0m\n"
    echo -e "\033[2mACTUAL:  \033[22m\033[34m $1\033[0m\n"
    echo -e "\033[2mEXPECTED:\033[22m\033[34m $2\033[0m"

    exit 1
}

cd /tmp/grit-test-repo
for FILE in `ls $(dirname)/assertions | sort -g`; do
    if [[ $FILE == *.test.sh ]]; then :
    else continue
    fi

    ASSERTIONS=$((ASSERTIONS + 1))

    echo -e "\n\033[34m• $ASSERTIONS\033[0m\n\033[2m"

    source "$(dirname)/assertions/$FILE"

    echo -e "\033[0m\033[32m✓ \033[0m\033[2m–––––––––––––––––\033[0m"
done