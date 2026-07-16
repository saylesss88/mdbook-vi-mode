use clap::{Arg, Command};

#[must_use]
pub fn make_app() -> Command {
    Command::new("vi-mode")
        .about("A mdBook preprocessor that implements vi-mode for mdBook")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}
