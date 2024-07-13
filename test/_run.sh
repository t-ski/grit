ASSERTIONS=0

test_dir() {
    echo $(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
}

grit() {
    $(test_dir)/../target/debug/grit $@
}

assert() {
    if [[ $1 == $2 ]]; then
        return
    fi

    echo -e "\033[0m"
    echo -e "\033[31mInaccurate assertion:\033[0m\n"
    echo -e "\033[2mACTUAL:  \033[22m\033[34m $1\033[0m\n"
    echo -e "\033[2mEXPECTED:\033[22m\033[34m $2\033[0m"
    
    exit 1
}

echo -e "\033[2m\033[1mBUILD\033[0m"
cargo build --manifest-path="$(test_dir)/../Cargo.toml"
echo ""

cd /tmp/grit-test-repo
for FILE in `ls $(test_dir)/assertions | sort -g`; do
    if [[ $FILE == *.test.sh ]]; then :
    else continue
    fi

    ASSERTIONS=$((ASSERTIONS + 1))

    echo -e "\033[2m–––––––––––––––––––\033[0m"
    echo -e "\033[34m• $ASSERTIONS\033[0m\n\033[2m"

    source "$(test_dir)/assertions/$FILE";
done