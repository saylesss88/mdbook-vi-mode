use clap::ArgMatches;
use mdbook_preprocessor::{errors::Result, Preprocessor};
use mdbook_vi_mode::ViMode;
use semver::{Version, VersionReq};
use std::{io, process};

/// Reads a book and its context from stdin, runs the given preprocessor
/// over it, and writes the processed book back out to stdout as JSON.
///
/// Also checks the mdbook version reported in the context against the
/// version this crate was built against, emitting a warning (not an
/// error) on mismatch.
///
/// # Errors
///
/// Returns an error if:
/// - stdin cannot be read, or its contents cannot be deserialized into
///   the expected `(Context, Book)` structure
/// - `ctx.mdbook_version` is not a valid semver version string
/// - `pre.run` fails, for any reason specific to the given preprocessor
/// - the processed book cannot be serialized, or cannot be written to
///   stdout
pub fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook_preprocessor::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

pub fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");

    let supported = pre.supports_renderer(renderer).unwrap_or_else(|err| {
        eprintln!("Error checking renderer support: {err}");
        process::exit(2);
    });

    // Signal whether the renderer is supported by exiting with 1 or 0.
    process::exit(i32::from(!supported));
}
