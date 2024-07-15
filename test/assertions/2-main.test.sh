### Write and stage a file
echo "." >> /tmp/grit-test-repo/contents.txt
git add .
assert "$(git status | head -7 | tail -1 | xargs)" "new file: contents.txt"

### Commit staged file
git commit -m "Test"
assert "$(git status | head -3 | tail -1)" "nothing to commit, working tree clean"