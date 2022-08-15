use butler::error::ButlerError;
use butler::butler::config::Config;
use butler::butler::ButlerApp;
use butler::member::ButlerType;

fn main() -> Result<(), ButlerError> {
    let config = Config::load(ButlerType)?;

    ButlerApp::new(config.addr).run()?;

    Ok(())
}

