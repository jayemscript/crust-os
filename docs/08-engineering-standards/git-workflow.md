# Git workflow

Keep changes scoped and reviewable. Do not initialize Git or rewrite history as part of the documentation foundation. Future contributors should inspect the working tree before editing, preserve unrelated user changes, and include only intentional files in a commit.

Use a branch for an authorized implementation phase when appropriate. Review the diff for generated output, secrets, binary images, logs, and .pnpm-store before sharing changes. Never generate or commit .pnpm-store. The root [.gitignore](../../.gitignore) is a safety net, not permission to create artifacts.

Commit messages should identify the affected phase or documentation topic. A review should link the phase plan and decision record, and report actual validation rather than an assumed result.
