// Minimal placeholder for journal_entry to allow crate to compile in CI/checks.
// Real implementation lives elsewhere in the design docs.

/// Placeholder type exported for build completeness.
#[derive(Debug, Clone)]
pub struct JournalEntryPlaceholder;

impl JournalEntryPlaceholder {
    /// noop constructor
    pub fn new() -> Self {
        JournalEntryPlaceholder
    }
}
