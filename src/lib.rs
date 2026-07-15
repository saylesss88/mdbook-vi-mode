use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::{Error, Result},
    Preprocessor, PreprocessorContext,
};

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
    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {}
    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}
