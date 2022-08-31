use butler_cli::butler_cli::config::Config;
use butler_cli::butler_cli::ButlerCliApp;
use common::app::ServantApp;
use common::app::ServantResult;
use common::config::ConfigReader;

fn main() -> ServantResult<()> {
    let config = Config::load("butler_cli.conf".to_owned())?;

    ButlerCliApp::new(config.butler_addr).run()?;

    Ok(())
}
