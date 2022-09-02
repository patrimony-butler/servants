use butler::butler::config::ConfigData;
use butler::butler::ButlerApp;
use common::app::ServantApp;
use common::app::ServantResult;
use common::config::ConfigLoader;

fn main() -> ServantResult<()> {
    let config = ConfigData::load("butler.conf".to_owned())?;

    ButlerApp::new(config.addr).run()?;

    Ok(())
}
