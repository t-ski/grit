### Start major development while on minor (failure)
grit major --name experimental ignored-arg
assert "$(git rev-parse --abbrev-ref HEAD)" "minor/test"

### Abort current minor development
grit abort
assert "$(git rev-parse --abbrev-ref HEAD)" "main"

### Start major development
grit major --name experimental ignored-arg
assert "$(git rev-parse --abbrev-ref HEAD)" "major/experimental"