use butler_cli::butler_cli::config::ConfigData;
use butler_cli::butler_cli::ButlerCliApp;
use common::app::ServantApp;
use common::app::ServantResult;
use common::config::ConfigLoader;

fn main() -> ServantResult<()> {
    let config = ConfigData::load("butler_cli.conf".to_owned())?;

    ButlerCliApp::new(config.butler_addr).run()?;

    Ok(())
}
