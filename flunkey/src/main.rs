use common::app::ServantApp;
use common::app::ServantResult;
use common::config::ConfigLoader;
use flunkey::flunkey::config::ConfigData;
use flunkey::flunkey::FlunkeyApp;

fn main() -> ServantResult<()> {
    let config = ConfigData::load("flunkey.conf".to_owned())?;

    FlunkeyApp::new(config.butler_addr).run()?;

    Ok(())
}
