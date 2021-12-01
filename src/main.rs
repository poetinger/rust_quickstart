mod arguments;

use structopt::StructOpt;
use arguments::{Arguments, Subcommand};

{% if ask_for_logging %}
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
{% endif %}

{% if ask_for_errors %}
fn main() -> Result<(), anyhow::Error> {
{% else %}
fn main() {
{% endif %}
    // Capture command line arguments.
    let Arguments { subcommand, verbose, ..} = Arguments::from_args();

    {% if ask_for_logging %}
    // Perform setup.
    setup(verbose);
    {% endif %}

    match subcommand {
        Subcommand::Run => {
            {% if ask_for_logging %}
            log::info!("Running...");
            {% else %}
            println!("Running...");
            {% endif %}
        },
    }
    {% if ask_for_errors %}

    Ok(())
    {% endif %}
}
