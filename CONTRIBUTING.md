
# Contributing

Thank you for wanting to contribute to EpilogLite. This file explains how to propose changes, the project's expectations for code and commits, and where to run checks locally before opening a pull request.

## Quick links

- Code of Conduct: `CODE_OF_CONDUCT.md`
- Security policy: `SECURITY.md` (report vulnerabilities privately)
- Developer Certificate of Origin: `DCO.md`
- GitHub templates and CODEOWNERS: `.github/`
- Design docs: `docs/design/`
- License files: `LICENSE.md`, `LICENSE-MIT.md`, `LICENSE-Apache.md`

If anything below is unclear or you need help, open a discussion or an issue and tag @JEleniel.

## Before you start

- Read the `docs/design/README.md` and the relevant design documents for the area you intend to change.
- Search open issues and discussions — you may find that someone else is already working on the same problem.
- If you plan a large or invasive change, open an issue or discussion first to get maintainers' feedback on the approach.

## Code of Conduct and Security

- All contributors must follow `CODE_OF_CONDUCT.md`.
- Do not report security vulnerabilities in public issues. Follow `SECURITY.md` to disclose vulnerabilities privately.

## Branches and naming

- Create a short, descriptive branch name, for example: `fix/pager-serialization`, `feat/rowid-index`, or `docs/sql-syntax`.
- Base feature branches on the project's default branch (`main`).

## Pull requests

- Keep PRs focused and single-purpose. Large changes should be split into smaller PRs.
- Fill the PR template in `.github/pull_request_template.yml` — it helps reviewers and CI.
- Link related issues or discussions in your PR description.
- Add tests and documentation changes alongside functional changes.
- Request reviews from maintainers listed in `.github/CODEOWNERS` when appropriate.
- Rebase interactively to keep history clean; squash trivial fixup commits before merge.

Recommended PR checklist (add to PR description or use the template):

- [ ] Code follows rustfmt defaults (`cargo fmt`)
- [ ] New and existing unit tests pass locally
- [ ] Documentation updated where applicable
- [ ] Commit messages follow the project's guidelines and DCO

### Changes to repository configuration

- Changes to `.github/` (workflows, templates, CODEOWNERS) affect contributor experience and CI; open a PR and request explicit review from maintainers.

## Local checks (what to run before opening a PR)

Run these commands locally and fix any problems they report. These are the same checks CI runs for PRs.

```bash
# Format check
cargo fmt --all -- --check

# Static analysis
cargo clippy --workspace --all-targets --all-features

# Build and tests
cargo test --workspace --all-features

# Quick build check (optional)
cargo check --workspace --all-features

# Optional: coverage helper (may require local tooling)
./coverage.sh || true
```

Notes:

- CI may run with additional flags or feature sets; passing locally reduces iteration time but CI is the final gate.

## Tests

- Add unit tests next to the code they exercise. Place integration tests in the relevant crate's `tests/` directory.
- Keep tests deterministic. Use `-- --nocapture` only for debugging locally.

## Commits and signing

- All contributions must include a DCO sign-off. See `DCO.md` for details. You can add a sign-off with `git commit -s`.
- We prefer concise commit messages in the imperative mood. Consider using the Conventional Commit style:

Good examples:

```text
feat(persistence): add rowid index for fast lookups
fix(pager): correct checksum calculation to include slot_index
docs: document page format in docs/design/Storage_and_Pages.md
```

Bad examples:

```text
fixed stuff
WIP
quick fix
```

If you sign commits with GPG, ensure your signature is attached to the commits pushed to GitHub.

## Code standards and style

- Follow Rust idioms and the project style (use `rustfmt` and `clippy`).
- Keep functions small and focused; add comments for complex algorithms.
- Add or update documentation and design docs for non-trivial changes. See `docs/design/` for conventions (Mermaid diagrams, RFC 2119 keywords, etc.).

## Documentation contributions

- Small documentation fixes can be made directly on a branch and opened as a PR.
- For design-level changes, update the appropriate file in `docs/design/` and follow the document conventions in `docs/design/README.md`.

## Issues and discussions

- Use Issues to report bugs and request small features. Provide reproduction steps and environment details when possible.
- Use Discussions for design proposals, broad feature requests, or questions that require community feedback.

## License

The project is licensed under MIT OR Apache-2.0 (your choice). See the top-level
license files (`LICENSE-MIT.md` and `LICENSE-Apache.md`) for full text and licensing details.

When contributing, ensure any added third-party dependencies are compatible with MIT/Apache-2.0.

## Thank you

Thanks for contributing! We review PRs as promptly as we can. If your PR needs help or edits, we'll leave constructive feedback to guide you.
