mod arguments;

use structopt::StructOpt;
use arguments::{Arguments, Subcommand};

/// Setup the application environment.
fn setup(verbose: u8) {
    // Make panics log to the logger.
    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("{}", panic_info);
    }));

    // Initialize logger.
    pretty_env_logger::formatted_builder()
        .filter_level(match verbose {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();
    log::trace!("Verbose logging enabled.");
}

fn main() -> Result<(), anyhow::Error> {
    // Capture command line arguments.
    let Arguments { subcommand, verbose, ..} = Arguments::from_args();

    // Perform setup.
    setup(verbose);

    match subcommand {
        Subcommand::Run => {
            log::info!("Running...");
            Ok(())
        },
    }
}
