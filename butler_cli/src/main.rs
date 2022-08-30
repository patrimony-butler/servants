use butler_cli::butler_cli::config::Config;
use butler_cli::butler_cli::ButlerCliApp;
use common::app::ServantApp;
use common::config::ConfigReader;
use common::error::ServantError;

fn main() -> Result<(), ServantError> {
    let config = Config::load("butler_cli.conf".to_owned())?;

    ButlerCliApp::new(config.butler_addr).run()?;

    Ok(())
}
