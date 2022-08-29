use common::config::ConfigReader;
use common::error::ButlerError;
use common::member::ConfigResolver;
use flunkey::config::FlunkeyType;
use flunkey::flunkey::config::Config;
use flunkey::flunkey::FlunkeyApp;

fn main() -> Result<(), ButlerError> {
    let config = Config::load(FlunkeyType::new("flunkey.conf".to_owned()))?;

    FlunkeyApp::new(config.butler_addr).run()?;

    Ok(())
}
