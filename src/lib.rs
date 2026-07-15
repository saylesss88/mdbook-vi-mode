use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::{Error, Result},
    Preprocessor, PreprocessorContext,
};

const VI_MODE_CSS: &str = include_str!("../vi-mode.css");
const VI_MODE_JS: &str = include_str!("../vi-mode.js");

pub struct ViMode;

impl Default for ViMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ViMode {
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

        // A distinct HTML block: the leading blank line makes pulldown-cmark
        // treat it as raw HTML and pass it through verbatim into the page.
        let payload =
            format!("\n\n<style>\n{VI_MODE_CSS}\n</style>\n<script>\n{VI_MODE_JS}\n</script>\n");

        // Each chapter renders to its own standalone .html page, so the payload
        // must land on every one of them.
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
