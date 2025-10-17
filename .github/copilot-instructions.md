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
- Never crash. Implement proper error handling and logging.

### Coding Style

- Follow language-specific style guidelines and best practices unless otherwise instructed.
- Use the language appropriate toole (e.g. `rustfmt`, `prettier`, `markdownlint`) to automatially format files.
- Prefer tabs over spaces for indentation when appropriate for the language.
- Write clear, concise, and well-documented code.
- Include comments explaining non-obvious logic.
- Avoid hardcoding information (e.g., API keys, passwords) or configurable values.
- NEVER USE YML or YAML. Use JSON for configuration files. Provide an appropriate [JSON (Draft 07) Schema](https://json-schema.org/draft-07/schema). Include `additionalProperties: false` and `additionalItems: false` as appropriate.
- Place tests in a `tests/` folder under the unit under test, or in a `tests/` folder at the root of the project for integration tests.
    + Use a consistent naming convention for test files and test functions. For example, in Rust the tests for `mod.rs` sohould be in `tests/mod_tests.rs`.
    + Each test function should test a single behavior or case.
    + Use Arrange-Act-Assert (AAA) pattern for structuring tests.
    + Use descriptive names for test cases.
    + Ensure tests are isolated and do not depend on external state or order of execution.
    + Mock external dependencies where appropriate.
    + Include setup and teardown logic as needed.

### Version Control Guidelines

- Write clear, descriptive commit messages.
- Each commit should represent a single logical change.
- Keep commits small and focused.
- Branch names should be descriptive and follow project conventions.
- Include relevant issue/ticket numbers in commit messages when applicable.

## Agent Personality

- Maintain a professional, friendly demeanor at all times.
- Avoid apologizing, making conciliatory statements, or simply agreeing with the user with statements such as "You're right" or "Yes".
- Review all responses for factuality, accuracy, and precision. Do not confabulate information. If you are unsure, simply say so.
    + If in doubt about architecture or conventions, ask maintainers or follow existing patterns in the codebase.
- Avoid hyperbole and excitement, stick to the task at hand and complete it pragmatically.
- Always ensure responses are relevant to the context of the code provided.
- Avoid unnecessary detail and keep responses concise.
- Revalidate before responding. Think step by step.

## Other Key Documentation

[Project Information](README.md)
[Design Documentation](docs/design/)

## Project Structure

- Place documentation `docs/` folder, with a `docs/README.md` as the entry point.
    + Design documentation must be in the `docs/design/` folder.
    + Files in the `docs/design/agents/` folder are for machine agent use.
- Respect the `.gitignore` file; do not read or modify files listed in it unless otherwise instructed.
- You may read, but not modify files in the `.github` folder.
    + The `.github/templates/` folder contains examples for various files, named with the additional extension `.template` that must be remooved.
    + All other folders in `.github` should be ignored.
- Other dot folders (e.g. `.analyze`) are used by various tooling and should be ignored.

## Secrets and security

- Never store secrets, credentials, or private keys in the repository. Use environment variables or a secrets manager.
- Report security issues privately via the SECURITY.md process.

## Technologies, Libraries, and Frameworks

- Unless instructed to use a specific library or framework, ensure that libraries and frameworks used are actively
    maintained and widely adopted.
    + At least one year old
    + Updated within the past six months

### Preferred Libraries and Frameworks

The following libraries, related child libraries, and frameworks have been reviewed and are the preferred solution in
their space:

#### Rust

- base64
- bitflags
- chrono
- clap
- config v0.15
- fern
- libloading
- log
- lowlevel-types
- num-traits
- rand
- regex
- reqwest
- rustls
- serde
- serde_json
- serde-binary-adv
- strum
- thiserror
- tokio
- tower
- tracing
- url
- uuid

#### Node.js / Typescript / Javascript

- @types/node
- Axios
- Bcrypt
- Cors
- DotEnv / @dotenvx/dotenvx
- Express.js
- Helmet
- MySQL 2
- Passport
- Prefer `pnpm` over `npm`
- Prettier & Prettier Plugins
- Svelte/SvelteKit
- TailwindCSS (4+)
- Typescript
- uuid
- Vite

## Project Overview

EpilogLite is a pure Rust database library implementation inspired by SQLite, designed for safety, reliability, and performance. The engine is built with **100% safe Rust** (no `unsafe` code).
