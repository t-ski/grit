echo "." >> /tmp/grit-test-repo/contents.txt
git add .
git commit -m "Test"

assert "$(git status | head -3 | tail -1)" "nothing to commit, working tree clean"