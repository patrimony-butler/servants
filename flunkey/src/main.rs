use common::error::ButlerError;
use common::member::FlunkeyType;
use flunkey::flunkey::config::Config;
use flunkey::flunkey::FlunkeyApp;

fn main() -> Result<(), ButlerError> {
    let config = Config::load(FlunkeyType)?;

    FlunkeyApp::new(config.butler_addr).run()?;

    Ok(())
}
