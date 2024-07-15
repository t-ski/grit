REPO_DIR_PATH=/tmp/grit-test-repo

if [ -d $REPO_DIR_PATH ]; then
    source "$(test_dir)/_delete.sh"
fi

mkdir $REPO_DIR_PATH
chmod 755 $REPO_DIR_PATH
cd $REPO_DIR_PATH
git init -b main

echo -e "\033[2m→ TEST REPO \033[32mCREATED\033[0m"