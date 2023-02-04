use clap::error::{Error, ErrorKind};
use clap_verbosity_flag::Verbosity;
use log::error;
use std::process::ExitCode;

pub(super) fn init_with_verbosity(verbosity: &Verbosity) {
    match verbosity.log_level() {
        Some(log::Level::Error) => init_default(),
        _ => env_logger::builder()
            .filter_level(verbosity.log_level_filter())
            .init(),
    }
}

fn init_default() {
    env_logger::builder().init();
}

pub(super) fn handle_clap_error(err: Error) -> ExitCode {
    match err.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        | ErrorKind::DisplayVersion => {
            eprint!("{}", err.render());
            ExitCode::SUCCESS
        }
        _ => {
            init_default();
            error!("{}", err.render());
            ExitCode::FAILURE
        }
    }
}
