---
applyTo: '**/*'
---

# IDE Specific Instructions

- You are working within the Visual Studio Code IDE.
- The specified tools, such as `markdownlint` and `prettier`, are installed globally. If you encounter any issues running them, notify the user.

## Memory Management

When context usage approaches 75%, hand off to a new session with this message: "Read your memory, `.github/copilot-instructions.md`, and `AGENT_PROGRESS.md` to resume".

Note: Some tooling cannot open files above a fixed size limit (for example, ~50MB). Keep `AGENT_PROGRESS.md` below that limit by de-duplicating and compressing it as needed.

## Prohibited Actions

You MUST NOT, at any time, for any reason, perform any of the following actions:

- Attempt to write to any folder outside of the workspace. If you need a temporary working space, create a `tmp/` folder in the workspace.
- Use `|| true` or `true ||` or `true` as a command or part of a command, especially in the terminal.
- Use the `gh` command line tool. **It is not installed and will not be.**
- Use the `head` or `tail` commands in the terminal.

## Memory

- You are equipped with a local memory capability.
- You MUST begin every session by reading your memory, no exceptions.
- If your memory exceeds 500 lines or 10% (whichever is smaller) of the total context window, compress it by summarizing key points and removing redundancies.

## Additional Instructions

- If you are the Architect agent, run the `tools/aurora_generator/aurora_generator` to generate the human friendly Markdown files. `./tools/aurora_generator/aurora_generator docs/example/aurora/`
