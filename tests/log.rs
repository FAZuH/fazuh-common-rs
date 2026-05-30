#[cfg(all(feature = "log", feature = "log-file"))]
mod log_file_tests {
    use std::fs;

    use fazuh_common::log::FileAppenderConfigBuilder;
    use fazuh_common::log::LogConfigBuilder;
    use fazuh_common::log::install_logging;

    #[test]
    fn test_logging_to_file() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let log_dir = temp_dir.path().to_path_buf();

        let file_config = FileAppenderConfigBuilder::default()
            .dir(log_dir.clone())
            .prefix("test_log".to_string())
            .suffix("log".to_string())
            .max_files(3)
            .build()
            .unwrap();

        let config = LogConfigBuilder::default()
            // Set to INFO level
            .verbosity(1)
            .ansi(false)
            .file_appender(file_config)
            .build()
            .unwrap();

        let guard = install_logging(config);

        tracing::trace!("Invisible trace message");
        tracing::debug!("Invisible debug message");
        tracing::info!("Visible info message");
        tracing::error!("Visible error message");

        // Flush and drop the guard
        drop(guard);

        // Find the generated log file
        let mut log_file_path = None;
        for entry in fs::read_dir(&log_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                log_file_path = Some(path);
                break;
            }
        }

        let log_file_path = log_file_path.expect("Log file was not created");
        let contents = fs::read_to_string(log_file_path).unwrap();

        assert!(!contents.contains("Invisible trace message"));
        assert!(!contents.contains("Invisible debug message"));
        assert!(contents.contains("Visible info message"));
        assert!(contents.contains("Visible error message"));
    }
}

#[cfg(feature = "log-cli")]
mod cli_tests {
    use fazuh_common::log::cli::ColorMode;
    use fazuh_common::log::cli::get_ansi;

    #[test]
    fn test_color_mode_always() {
        // Just verify basic logic: calling it shouldn't panic.
        // Whether it returns true or false depends on ambient env vars (like NO_COLOR),
        // but we can at least assert it runs.
        let ansi_always = get_ansi(ColorMode::Always);

        if std::env::var_os("NO_COLOR").is_none() && std::env::var_os("CLICOLOR").is_none() {
            assert!(
                ansi_always,
                "ColorMode::Always should be true when no env var overrides exist"
            );
        }

        let ansi_never = get_ansi(ColorMode::Never);
        assert!(!ansi_never, "ColorMode::Never should always be false");
    }
}
