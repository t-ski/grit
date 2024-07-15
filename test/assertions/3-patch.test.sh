### Start patch development
grit patch --name test
assert "$(git rev-parse --abbrev-ref HEAD)" "patch/test"

### Write a file and complete patch
echo "." >> /tmp/grit-test-repo/contents.txt
grit complete
# assert "$(git branch | head -1 | tail -1)" "* main"
# assert "$(git branch | head -2 | tail -1)" ""