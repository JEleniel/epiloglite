---
applyTo: '**/*'
---

# Coding Instructions

General coding expectations now live in `.github/copilot-instructions.md` under **General Coding Guidelines** and **Agent Behavior**. Treat those sections as authoritative.

Additional repository-wide reminders:

- Never disable checks or tests (e.g., `// @ts-nocheck`, `#[allow(...)]`). Fix the underlying issue instead.
- Unimplemented paths must still fail fast and clearly communicate intent (`todo!`, `unimplemented!`, etc.).
- Do not label code “production ready”; rely on the review + release process instead.
