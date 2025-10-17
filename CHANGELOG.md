# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Added `.github/PULL_REQUEST_TEMPLATE.md` for standardized pull request submissions.
- Added `.github/instructions/CodingStandards.instructions.md` for detailed coding practices and style guidelines.
- Tests: expanded `epiloglite-core` serialization utility tests. Added round-trip tests for `try_into_vec`/`try_from_slice`, `serialized_size` checks, explicit error-variant assertions for `SerializeError` (Decode, Encode and CrcInputEmpty cases), and an encode-failure test for `try_to_writer`.

### Notes

### Changed

### Removed

- Deleted `.github/ARCHITECTURE_TEMPLATES/Requirements.md` (requirements template moved or deprecated).
- Deleted `.github/instructions/Global.instructions.md` (replaced by more specific instructions).

[Unreleased]: https://github.com/jeleniel/epiloglite/compare/v1.0.0...HEAD
