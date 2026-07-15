//! `mdbook-vi-mode` is an [mdBook] preprocessor that adds Vim-style keyboard
//! navigation to the rendered HTML book.
//!
//! # How it works
//!
//! A preprocessor runs *before* rendering and can only transform the book's
//! Markdown content; it has no access to the final page or any notion of a
//! cursor. The navigation itself is therefore implemented in client-side
//! JavaScript, and this preprocessor's only job is to inject that script (plus
//! its stylesheet) into every chapter so it ships on every rendered page.
//!
//! Because mdBook renders each chapter as a standalone HTML page that always
//! includes the sidebar, the injected script can move a single cursor between
//! the chapter list (`#mdbook-sidebar a`) and the page body
//! (`#mdbook-content main`).
//!
//! # Configuration
//!
//! All options live under `[preprocessor.vi-mode]` in `book.toml` and are
//! optional:
//!
//! ```toml
//! [preprocessor.vi-mode]
//! toggle-key   = "`"        # key that turns navigation on and off
//! start-active = false      # whether the cursor is shown on page load
//! cursor-color = "#e46876"  # any CSS color
//! ```
//!
//! [mdBook]: https://rust-lang.github.io/mdBook/

use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem},
    errors::{Error, Result},
};
use serde::Deserialize;

/// The client-side stylesheet, embedded at compile time.
const VI_MODE_CSS: &str = include_str!("../vi-mode.css");
/// The client-side navigation script, embedded at compile time.
const VI_MODE_JS: &str = include_str!("../vi-mode.js");

/// User-facing options read from the `[preprocessor.vi-mode]` table.
///
/// Missing fields fall back to [`ViConfig::default`], so an empty table — or no
/// table at all — yields sensible defaults. Unknown keys (such as mdBook's own
/// `command` or `renderers` entries) are ignored.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct ViConfig {
    /// The [`KeyboardEvent.key`] value that toggles navigation on and off.
    ///
    /// [`KeyboardEvent.key`]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    toggle_key: String,
    /// Whether the cursor is visible immediately on page load.
    start_active: bool,
    /// The CSS color used for the cursor outline and sidebar highlight.
    cursor_color: String,
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

/// The `mdbook-vi-mode` preprocessor.
///
/// It carries no state of its own; per-book configuration is read from the
/// [`PreprocessorContext`] each time [`run`](Preprocessor::run) is called.
#[derive(Debug, Clone, Copy, Default)]
pub struct ViMode;

impl ViMode {
    /// Creates a new [`ViMode`] preprocessor.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Preprocessor for ViMode {
    /// Returns the preprocessor name used in `book.toml`
    fn name(&self) -> &'static str {
        "mdbook-vi-mode"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        // Only inject for the HTML renderer; other renderers (e.g. a linkcheck
        // pass) also call run(), and raw <script> would be noise there.
        if ctx.renderer != "html" {
            return Ok(book);
        }

        // Read config once per run. Invalid config is a warning, not a build
        // failure: it is better to fall back to defaults than to break `mdbook
        // build` over a typo in an optional table.
        let config = match ctx.config.get::<ViConfig>("preprocessor.vi-mode") {
            Ok(Some(config)) => config,
            Ok(None) => ViConfig::default(),
            Err(err) => {
                eprintln!("[{}] ignoring invalid configuration: {err}", self.name());
                ViConfig::default()
            }
        };

        let payload = build_payload(&config);

        // `for_each_mut` recurses through nested `sub_items`, so every chapter
        // (each of which becomes its own standalone HTML page) gets the payload.
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ch) = item {
                ch.content.push_str(&payload);
            }
        });

        Ok(book)
    }
    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}

/// Builds the `<style>`/`<script>` block appended to every chapter.
///
/// The configuration is serialized into a small `window.__viModeConfig` object
/// that the bundled script reads at runtime. This keeps [`VI_MODE_JS`] a plain,
/// self-contained file with its own built-in defaults, so it can be linted and
/// tested on its own.
///
/// The leading blank line matters: it makes pulldown-cmark treat the block as
/// raw HTML and pass it through verbatim rather than escaping it.
fn build_payload(config: &ViConfig) -> String {
    // `serde_json` guarantees the string values are correctly escaped before
    // being embedded in the inline <script>.
    let toggle_key =
        serde_json::to_string(&config.toggle_key).unwrap_or_else(|_| "\"`\"".to_owned());
    let cursor_color =
        serde_json::to_string(&config.cursor_color).unwrap_or_else(|_| "\"#e46876\"".to_owned());

    format!(
        "\n\n<style>\n{css}\n</style>\n\
         <script>\nwindow.__viModeConfig = {{ \
         \"toggleKey\": {toggle_key}, \
         \"startActive\": {start_active}, \
         \"cursorColor\": {cursor_color} }};\n</script>\n\
         <script>\n{js}\n</script>\n",
        css = VI_MODE_CSS,
        start_active = config.start_active,
        js = VI_MODE_JS,
    )
}
