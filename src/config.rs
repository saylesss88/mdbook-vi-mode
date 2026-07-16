use serde::Deserialize;

/// User-facing options read from the `[preprocessor.vi-mode]` table.
///
/// Missing fields fall back to [`ViConfig::default`], so an empty table — or no
/// table at all — yields sensible defaults. Unknown keys (such as mdBook's own
/// `command` or `renderers` entries) are ignored.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ViConfig {
    /// The [`KeyboardEvent.key`] value that toggles navigation on and off.
    ///
    /// [`KeyboardEvent.key`]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    pub toggle_key: String,
    /// Whether the cursor is visible immediately on page load.
    pub start_active: bool,
    /// The CSS color used for the cursor outline and sidebar highlight.
    pub cursor_color: String,
}

impl Default for ViConfig {
    fn default() -> Self {
        Self {
            toggle_key: "`".to_owned(),
            start_active: false,
            cursor_color: "#e46876".to_owned(), // Kanagawa waveRed
        }
    }
}
