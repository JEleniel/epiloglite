# Agent Instructions

His praeceptis sine exceptione pare.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119 and updated in RFC 8174.

Instructions apply in the following precedence order (earlier entries override later ones):

1. System Instructions (including safety policies and tooling constraints)
2. User Instructions (nothing overrides user intent except System Instructions)
3. Inline comment instructions
4. Language-specific and applicable `.github/instructions/*.instructions.md`
5. Repo Instructions (this file)
6. Tool defaults and generated templates

If tooling limitations or system instructions prevent compliance, you MUST stop and notify the user of the conflict.

## Agent Manifest

- The canonical, machine-readable definition of every agent (model, scope, owned areas, deliverables, and review targets) lives in `.github/agents/agent_manifest.json`.
- Each `.agent.md` file may focus on responsibilities and nuances; defer to the manifest for metadata instead of duplicating it elsewhere.

## General Coding Guidelines

- You MUST use relative paths for local files unless absolutely necessary (e.g., system paths, tooling requirements, etc.). Links in documentation MUST be relative to the document.
- You MUST conform to best practices for the language you are coding in. Language-specific configuration files (e.g., `rustfmt.toml`, `.markdownlint.json`, and `.prettierrc.json`) are authoritative and override general style rules.
- You MUST use tabs whenever possible for indentation unless the formatter and associated configuration specify otherwise. Do not fight the formatter. If a file could use tabs but has spaces for indentation, keep the file consistent and report the exception to the user.
- You MUST organize code into logical modules that conform to the _single responsibility_ principle and the language-specific style. You SHOULD aim for a maximum of 20 lines per function, excluding boilerplate.
- You SHOULD aim for a maximum of approximately 200 lines per file. Modules SHOULD only contain a single primary structure and supporting elements _for that module only_. Shared supporting elements MUST be placed in separate files.
- You MUST use POSIX-style newlines (`\n`).
- You MUST use uppercase for hex literals. Other uses of hexadecimal should be consistent with idiomatic styles.
- Apply OWASP guidance and secure-by-design principles.
- Apply Twelve-Factor App principles.
- No global variables; global constants are allowed only in a dedicated constants file.
- Use descriptive names, full words, and verb-based function names (except standard getters/setters).
- Tests must prove behavior. Do not write null tests that only call functions without validation.

## Security by Default

- Prefer secure defaults over optional hardening.
- Validate all external inputs at trust boundaries.
- Follow least privilege: minimize permissions, exposed surfaces, and sensitive data retention.
- Never log secrets or sensitive payloads.

## Logging

- Use structured, leveled logs where possible.
- `TRACE`, `DEBUG`, `INFO`, and `WARN` should be suitable for standard output; `ERROR` should go to standard error when the runtime supports it.
- Log at boundaries (CLI entry points, request handlers, job runners) and avoid noisy logs in tight loops.

## Prohibited Actions

- You MUST NOT write or execute custom scripts, or run Python, Perl, or Node ad-hoc.
    + Any multi-line terminal input (anything that contains a newline, including heredocs) is a script.
    + A series of commands joined using pipeline (`|`) is a single instruction for this purpose and is allowed.
    + Do not use command chaining operators such as `&&` or `;`.
    + You may use approved tools (listed later in this file), shell commands, IDE tools, and MCP plugins.
- You MUST NOT branch from or open a PR to `main`.
- You MUST NOT use the `gh` command line tool. It is not installed.

## Work Tracking

The `.agents/PROGRESS.md` format has been deprecated, and split into a more organized, smaller format. It was previously located at Project Plan. If it is still there, move it to the new location, splitting it to the new format.

- You MUST maintain the `.agents/PROGRESS.md` (Progress Plan) file to track progress.
- When performing a review, you MUST create a `.agents/REVIEW-{TYPE}.md` file with all findings, mitigation guidance, and references. Link the review file from `.agents/PROGRESS.md`.
- See `.agents/PROJECT_BRIEF.md` for the canonical description of required `.agents/` files (PROJECT_BRIEF, PROGRESS, PATTERNS, TECHNOLOGIES, CONTEXT, and any review files).

### Plan Format Contract (All Agents)

The Planner owns the plan structure, but all agents must follow the same format when updating Project Plan:

- Add new work items using a stable identifier, short title, and explicit **Status**.
- Update only what you are responsible for:
    + Implementation agents: status transitions, links to relevant code, and next actions.
    + Review agents: findings, severity, and required next actions.
    + Documentation agents: documentation issue lists and required updates.
- Every item must include **Owner**, **Links** (Aurora card when applicable), and **Next Action**.

**Example Project Plan Feature Entry:**

The links in this example are illustrative; you MUST use actual links relevant to the project and relative to the Project Plan.

```markdown
-   [ ] **{{owner}}** ({id}) **{name}**
    * Status: {status}
    * Next Actions: ...
    * [Aurora Feature Card](.github/instructions/example_feature_card-11111111-1111-4111-8111-111111111111.json)
    * [GitHub Issue #123](https://github.com/example/repo/issues/123)
```

### Changelog

You MUST maintain a `CHANGELOG.md` file in the root of the repository that follows the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The changelog MUST be kept up to date with every change you make to the code. Documentation, the `.agents/` files, and the `.github/` folder MUST not be tracked in the `CHANGELOG.md`.

### Inline Comment Instructions and Edit Areas

Some files may have inline comments that start with `AGENT:` that provide specific instructions or mark areas where edits are allowed. You MUST follow these instructions exactly unless the user instructs otherwise and only make the instructed changes in the designated areas. The edit area will end with another `AGENT:` comment stating `End of edit area.`. Remove the contents when you finish the edits.

These inline comments may also provide additional context or requirements for the code in that file. You MUST read and understand these comments before making any changes.

The inline comments do not override direct instructions from the user. If the user provides instructions that conflict with the inline comments, you MUST follow the user's instructions.

## Project Structure & Ownership

See `.agents/PROJECT_BRIEF.md` for the authoritative ownership matrix, quick-start checklist, and required coordination files. Highlights:

- Modify `.github/` only when explicitly instructed (schema syncs are the sole standing exception).
- Respect role ownership for `docs/`, `docs/design/`, `tools/`, and language-specific source trees; work inside those areas only when acting in that role.
- You MAY update `.agents/*` and `CHANGELOG.md` as required by these instructions.

## Agent Behavior

- If an `docs/design/aurora/AGENT-*.json` file exists, read it to load the entire design.
- When a new technology or dependency is added or an existing one is changed (including when detected from someone else's changes), you MUST read the current documentation for the correct version and annotate the `AGENT_PROGRESS.md` with any notes needed to work safely and idiomatically.
- You MUST end final responses with a short summary paragraph, followed by a blank line, then **5-10 tl;dr bullets**. The last bullet MUST include an estimate of the current context usage as a percentage.
- You MUST make changes in small blocks, or use IDE or other approved tools for supported batch operations. You MUST NOT pause between files unless you need clarification or have been instructed to do so.
- Before opening or creating any file, you MUST read the relevant `*.instructions.md` files for that file type or language, if one exists.
- If you have any questions ask before beginning work, otherwise go dierectly to work. One started, continue until the work is complete unless absolutely necessary to stop.

## Tools

- You SHOULD use MCP interaction instead of command line or shell tools when possible.
- You MUST use the GitHub MCP for all GitHub interactions. If GitHub MCP is not available, stop and notify the user.
- You MUST use the Mermaid.js MCP to create and validate Mermaid diagrams.
- You MUST only run one command at a time; do not chain commands (e.g., `&&` or `;`).
- You MUST use `markdownlint`, 'prettier', and language specific tools for formatting and linting.

## Additional Guidelines

- You MUST NOT rely solely on git status or diffs to determine what has changed. You MUST track your own changes and ensure that you understand the full context of the project.
- Other agents and collaborators are also working on this project. Any changes you do not recognize were made by them. You MUST NOT revert changes you did not make.
- You MUST NOT pause or ask permission before making changes unless you are unsure about the requirements, need clarification, or have been instructed to do so.
- YAML linting instructions exist in both `.github/instructions/YAML.instructions.md` and `.github/instructions/YML.instructions.md`. The `.yml` variant references the `.yaml` document because the `applyTo` parser accepts only one extension per file.
