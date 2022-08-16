use butler::butler::config::Config;
use butler::butler::ButlerApp;
use butler::config::ButlerType;
use common::config::ConfigReader;
use common::error::ButlerError;

fn main() -> Result<(), ButlerError> {
    let config = Config::load(ButlerType)?;

    ButlerApp::new(config.addr).run()?;

    Ok(())
}
