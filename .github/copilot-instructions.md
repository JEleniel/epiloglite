# Copilot Instructions

All paths are relative to the repository root.

**Read and follow the instructions in the following files as part of these instructions:**

## Coding Practices and Style

### Priorities

1. Security
2. Robustness
3. Scalability
4. Performance
5. Maintainability

### Standards

Code must conform to:

- [The Twelve-Factor App](https://12factor.net/).
- [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/).
- [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/), if applicable.
- [OWASP Mobile Application Security Verification Standard (MASVS)](https://mas.owasp.org/MASVS/), if applicable.

### Acceptance Criteria

All code must:

- Compile with zero warnings or errors.
    + Future use code should be appropriately marked to avoid warnings (e.g. prefixed with `_` in Rust).
    + Unused code should be removed.
- Include passing unit tests for all generated functions and code.
    + Include positive and negative cases.
    + Include security tests, e.g. bad input handling.
- Use secure coding practices to prevent common vulnerabilities.

```markdown
# Copilot Instructions

All paths are relative to the repository root.

These instructions explain the project-level guidance Copilot and other automated agents should follow when contributing or suggesting changes. When in doubt about repository policy or modifying configuration in `.github/`, propose changes and create a PR for human review; do not push unilateral changes to repository metadata unless you have explicit maintainer approval.

## Priorities

1. Security
2. Robustness
3. Scalability
4. Performance
5. Maintainability

## Standards


When applicable, prefer established standards and best practices such as:

- The Twelve-Factor App: https://12factor.net/
- WCAG (accessibility) where relevant
- OWASP ASVS (if relevant to the scope)

## Acceptance Criteria

Generated or suggested code should meet these baseline expectations:

- Build and typecheck cleanly with zero errors; avoid introducing new compiler errors.
  - For Rust code, prefer following `rustfmt` defaults and `cargo clippy` guidance.
- Include unit tests for new behavior (positive and negative cases where appropriate).
  - Tests should mock external resources and stay focused on the unit under test.
- Follow secure-coding practices and handle errors robustly; avoid panics in library code.

## Formatting and Style

- Follow language-specific and tooling defaults (e.g., Rust: rustfmt; use rustfmt defaults rather than enforcing tabs/spaces manually).
- Use automated formatters where available (`rustfmt`, `prettier`, `markdownlint`).
- Prefer the repository's canonical format over personal preference. If a project-wide formatter is not configured, propose one (e.g., add rustfmt config).

Note: Configuration file format should follow the tool's expectations. While JSON is preferred for some config types, many tools (including GitHub Actions) require YAML and Rust uses TOML for Cargo. Do not replace format types required by tools.

## Tests and Coverage

- Aim for high test quality and meaningful coverage (for guidance, target >= 80% for critical modules), but prioritize correct, well-scoped tests over chasing a specific percentage.
- Organize tests using language conventions (for Rust, use module-level tests or `tests/` integration tests as appropriate).

## Version Control and Commits

- Write clear, descriptive commit messages. Keep commits focused and logically grouped.
- Branch names should be descriptive and follow project conventions.

## Agent Behavior & Personality

- Be professional, concise, and accurate.
- Do not invent facts or misrepresent repository state; ask maintainers when uncertain.
- Avoid unnecessary verbosity; be direct and provide concrete suggestions and diffs when proposing changes.

## Project Files and .github

- You may read files in `.github/` to understand repository policies.
- Do not modify `.github/` files directly without explicit maintainer approval. Instead, create a PR with proposed changes and include tests or validation where applicable.

## Secrets and Security

- Never commit secrets or private keys to the repository. Use environment variables or a secrets manager.
- Report security issues privately following the SECURITY.md guidance.

## Technologies and Libraries

- Prefer well-maintained libraries with recent activity. For Rust, examples of commonly used crates include `serde`, `tokio`, `regex`, `thiserror`, `uuid`, etc.

## Project Overview

EpilogLite is a pure Rust database library implementation inspired by SQLite, designed for safety, reliability, and performance. The engine aims to be 100% safe Rust (no `unsafe` code) where feasible.
