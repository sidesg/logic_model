use std::error::Error;
use std::env;
use std::process::exit;
use logic_model::model::Model;
use logic_model::configs::Config;


fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>>  {
    tracing_subscriber::fmt().init();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        tracing::error!("{}", err);
        exit(1);
    });
    let model = Model::from_file(config.infile()).unwrap_or_else(|err| {
        tracing::error!("{} ({})", err, config.infile());
        exit(1);
    });

    // model.eval_tableau();

    Ok(())
}
