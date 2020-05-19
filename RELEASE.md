# Release Procedure

Dependency: The "[Task](https://taskfile.dev/)" task runner

1. Update version number in Cargo.toml
2. Update version number in html_root_url attribute
3. Update CHANGELOG.md
4. Commit changes
5. Run `task release VERSION=a.b.c`, substituting the correct version