#[cfg(all(feature = "log", feature = "log-file"))]
use fazuh_common::log::FileAppenderConfigBuilder;
#[cfg(feature = "log")]
use fazuh_common::log::LogConfigBuilder;
#[cfg(all(feature = "log", feature = "log-cli"))]
use fazuh_common::log::cli::ColorMode;
#[cfg(all(feature = "log", feature = "log-cli"))]
use fazuh_common::log::cli::get_ansi;
#[cfg(feature = "log")]
use fazuh_common::log::install_logging;

fn main() {
    #[cfg(not(feature = "log"))]
    {
        println!("This example requires the 'log' feature.");
        println!("Run with: cargo run --example logging --features \"log,log-file,log-cli\"");
    }

    #[cfg(feature = "log")]
    {
        // 1. Setup Ansi using the CLI logic (if enabled)
        #[cfg(feature = "log-cli")]
        let ansi = get_ansi(ColorMode::Auto);
        #[cfg(not(feature = "log-cli"))]
        let ansi = true;

        // 2. Setup the File Appender Config (if enabled)
        #[cfg(feature = "log-file")]
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        #[cfg(feature = "log-file")]
        let log_dir = temp_dir.path().to_path_buf();

        #[cfg(feature = "log-file")]
        let _ = std::fs::create_dir_all(&log_dir);

        #[cfg(feature = "log-file")]
        let file_config = FileAppenderConfigBuilder::default()
            .dir(log_dir.clone())
            .prefix("example".to_string())
            .suffix("log".to_string())
            .max_files(3)
            .build()
            .unwrap();

        // 3. Build the core LogConfig
        let mut config_builder = LogConfigBuilder::default();
        config_builder.verbosity(2); // DEBUG level
        config_builder.ansi(ansi);

        #[cfg(feature = "log-file")]
        config_builder.file_appender(file_config);

        let config = config_builder.build().unwrap();

        // 4. Install the logging framework
        // Keep the guard alive for the duration of the program
        let _guard = install_logging(config);

        // 5. Emit some logs
        tracing::trace!("This is a TRACE message (might be filtered out)");
        tracing::debug!("This is a DEBUG message");
        tracing::info!("This is an INFO message");
        tracing::warn!("This is a WARN message");
        tracing::error!("This is an ERROR message");

        println!("\nLogging initialized.");
        #[cfg(feature = "log-file")]
        println!(
            "Check your console and {} for the log files!",
            log_dir.display()
        );
        #[cfg(not(feature = "log-file"))]
        println!(
            "Check your console for the log files. Run with 'log-file' feature to see file output."
        );

        // Ensure logs are written by dropping the guard explicitly before exiting
        drop(_guard);

        #[cfg(feature = "log-file")]
        {
            println!("The temporary directory will be deleted when this program exits.");
            println!("Wait 5 seconds to inspect...");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}
