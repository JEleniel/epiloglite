---
applyTo: '*.md'
---

# Markdown Style Guide

This document defines formatting and style conventions for Markdown files.

The source of truth for enforcement is `.markdownlint.json`.

## Guidelines

- You MUST use only ATX headings (`#`, `##`, …) and do not indent them.
- You MUST use a single H1.
- You MUST increase heading levels one at a time; do not skip levels.
    * Sibling headings must be unique.
- You MUST NOT use emphasis/strong as the entire heading text.
- You MUST NOT hard-wrap lines in paragraphs.
- You MUST NOT use extra spaces in prose (for example, multiple spaces between words). Tables are an exception where spacing inside cells may be used for readability or alignment.
- You MUST NOT manually align Markdown using spaces outside of tables. If you need alignment, use lists, tables, code blocks (tabs are fine in code), or omit the alignment.
- You MUST end files with exactly one trailing newline.
- You MUST avoid multiple blank lines together.
- Unordered lists MUST use `-` for the first level and `+` for nested lists to match `.markdownlint.json` (`ul-style: sublist`).
- Ordered lists MUST use sequential numbers.
- You MUST use fenced code blocks with backticks.
- You MUST include a language for all code blocks. Use `text` when no specific language applies.
- When documenting a command, include sample output.
- Do not use bare URLs; use Markdown links. Avoid reversed links. Do not create empty links.
- Link fragments (anchors) are allowed.
- Provide alternative text for images.
- You MAY use inline HTML but SHOULD avoid it unless it is necessary.
- You MUST use leading and trailing pipe characters for all table rows and ensure consistent column counts in every row.
- Keep blank lines around tables, code blocks, callouts, and headings.

## Frontmatter

- Markdown frontmatter MUST use `---` delimiters.
- Only include frontmatter when required by the consuming tool.

## Lists

- This repository uses `.markdownlint.json` as the source of truth for list indentation and markers.
- Unordered lists MUST follow the `sublist` behavior (`ul-style: sublist`). Nested list markers must differ from the parent list marker.
