use butler::butler::config::Config;
use butler::butler::ButlerApp;
use common::app::ServantApp;
use common::app::ServantResult;
use common::config::ConfigReader;

fn main() -> ServantResult<()> {
    let config = Config::load("butler.conf".to_owned())?;

    ButlerApp::new(config.addr).run()?;

    Ok(())
}
