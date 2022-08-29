use butler::butler::config::Config;
use butler::butler::ButlerApp;
use butler::config::ButlerType;
use common::config::ConfigReader;
use common::error::ButlerError;
use common::member::ConfigResolver;

fn main() -> Result<(), ButlerError> {
    let config = Config::load(ButlerType::new("butler.conf".to_owned()))?;

    ButlerApp::new(config.addr).run()?;

    Ok(())
}
