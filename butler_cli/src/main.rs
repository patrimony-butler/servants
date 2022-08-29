use butler_cli::butler_cli::config::Config;
use butler_cli::butler_cli::ButlerCliApp;
use butler_cli::config::ButlerCliType;
use common::config::ConfigReader;
use common::error::ButlerError;
use common::member::ConfigResolver;

fn main() -> Result<(), ButlerError> {
    let config = Config::load(ButlerCliType::new("butler_cli.conf".to_owned()))?;

    ButlerCliApp::new(config.butler_addr).run()?;

    Ok(())
}
