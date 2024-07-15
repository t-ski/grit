### Start minor development
grit minor --name test
assert "$(git rev-parse --abbrev-ref HEAD)" "minor/test"

### Write a file and complete minor
echo "." >> /tmp/grit-test-repo/contents.txt
grit complete
assert "$(git branch)" "* main"
assert "$(git rev-list --count HEAD)" "2"
assert "$(git describe --abbrev=0)" "v0.1.0"