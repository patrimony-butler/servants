use common::app::ServantApp;
use common::config::ConfigReader;
use common::error::ServantError;
use flunkey::flunkey::config::Config;
use flunkey::flunkey::FlunkeyApp;

fn main() -> Result<(), ServantError> {
    let config = Config::load("flunkey.conf".to_owned())?;

    FlunkeyApp::new(config.butler_addr).run()?;

    Ok(())
}
