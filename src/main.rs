pub mod app;
pub mod preprocessor;

use mdbook_vi_mode::ViMode;
use std::process;

fn main() {
    let matches = app::make_app().get_matches();

    let preprocessor = ViMode::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        preprocessor::handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = preprocessor::handle_preprocessing(&preprocessor) {
        eprintln!("{e:?}");
        process::exit(1);
    }
}
