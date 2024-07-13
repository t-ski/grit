ASSERTIONS=0

test_dir() {
    echo $(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
}

grit() {
    $(test_dir)/../target/debug/grit $@
}

assert() {
    ASSERTIONS=$((ASSERTIONS + 1))

    echo -e "\033[2m–––––––––––––––––––"
    echo -e "• $ASSERTIONS"

    if [[ $1 == $2 ]]; then
        echo -e "\033[0m"
        return
    fi

    echo -e "\033[0m"
    echo -e "\033[31mInaccurate assertion:\033[0m\n"
    echo -e "\033[2mACTUAL:  \033[22m\033[34m $1\033[0m\n"
    echo -e "\033[2mEXPECTED:\033[22m\033[34m $2\033[0m\n"
    
    exit 1
}

cargo build --manifest-path="$(test_dir)/../Cargo.toml"

cd /tmp/grit-test-repo
for FILE in `ls $(test_dir)/assertions | sort -g`; do
    if [[ $FILE == *.test.sh ]]; then :
    else continue
    fi
    
    source "$(test_dir)/assertions/$FILE";
done