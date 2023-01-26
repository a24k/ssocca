use clap_verbosity_flag::Verbosity;

pub fn init(verbosity: &Verbosity) {
    match verbosity.log_level() {
        Some(log::Level::Error) => env_logger::builder().init(),
        _ => env_logger::builder()
            .filter_level(verbosity.log_level_filter())
            .init(),
    }
}
