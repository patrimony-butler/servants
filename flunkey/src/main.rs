use common::app::ServantApp;
use common::app::ServantResult;
use common::config::ConfigReader;
use flunkey::flunkey::config::Config;
use flunkey::flunkey::FlunkeyApp;

fn main() -> ServantResult<()> {
    let config = Config::load("flunkey.conf".to_owned())?;

    FlunkeyApp::new(config.butler_addr).run()?;

    Ok(())
}
