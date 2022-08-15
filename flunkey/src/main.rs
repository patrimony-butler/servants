use common::error::ButlerError;
use flunkey::flunkey::config::Config;
use flunkey::flunkey::FlunkeyApp;
use common::member::FlunkeyType;

fn main() -> Result<(), ButlerError> {
    let config = Config::load(FlunkeyType)?;

    FlunkeyApp::new(config.butler_addr).run()?;

    Ok(())
}
