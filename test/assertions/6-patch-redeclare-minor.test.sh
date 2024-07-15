### Start patch development
grit patch --name test
assert "$(git rev-parse --abbrev-ref HEAD)" "patch/test"

### Redeclare to minor development
grit redeclare minor
assert "$(git rev-parse --abbrev-ref HEAD)" "minor/test"