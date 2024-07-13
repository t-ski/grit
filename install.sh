echo -e "\033[1mgrit\033[0m installing…"


if [ $# -eq 0 ]; then
    echo "Specify version (e.g. 1.0.0)"
    exit 1
fi

PATH=https://github.com/t-ski/grit/archive/refs/tags/v$1.tar.gz

output /dev/null --silent --head --fail $PATH
if [[ $? != 0 ]]; then
    echo "Could not retrieve data"
    exit 1
fi

curl $PATH -o grit.tar.gz
tar -xf grit.tar.gz
mv grit/target/release/grit /usr/local/bin/grit
rm -r grit.tar.gz

echo -e "\033[31mDone.\033[0m"