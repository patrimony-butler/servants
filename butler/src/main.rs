use butler::butler::config::Config;
use butler::butler::ButlerApp;
use common::app::ServantApp;
use common::config::ConfigReader;
use common::error::ServantError;

fn main() -> Result<(), ServantError> {
    let config = Config::load("butler.conf".to_owned())?;

    ButlerApp::new(config.addr).run()?;

    Ok(())
}
