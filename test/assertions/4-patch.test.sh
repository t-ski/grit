### Start patch development
grit patch -n test
assert "$(git rev-parse --abbrev-ref HEAD)" "patch/test"

### Write a file and complete patch
echo "." >> /tmp/grit-test-repo/contents.txt
grit complete
assert "$(git branch)" "* main"
assert "$(git rev-list --count HEAD)" "3"
assert "$(git describe --abbrev=0)" "v0.1.1"