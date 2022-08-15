use butler::error::ButlerError;
use butler::flunkey::config::Config;
use butler::flunkey::FlunkeyApp;
use butler::member::FlunkeyType;

fn main() -> Result<(), ButlerError> {
    let config = Config::load(FlunkeyType)?;

    FlunkeyApp::new(config.butler_addr).run()?;

    Ok(())
}

