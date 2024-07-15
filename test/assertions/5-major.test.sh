### Start major development
grit major --name test
assert "$(git rev-parse --abbrev-ref HEAD)" "major/test"

### Write a file and complete major
echo "." >> /tmp/grit-test-repo/contents.txt
grit complete
assert "$(git branch)" "* main"
assert "$(git rev-list --count HEAD)" "4"
assert "$(git describe --abbrev=0)" "v1.0.0"