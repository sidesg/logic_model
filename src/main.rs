use std::error::Error;
use std::env;
use logic_model::Model;
use logic_model::configs::Config;


fn main() -> Result<(), Box<dyn Error>>  {
    let config = Config::build(env::args())?;
    let model = Model::from_file(config.infile())?;
    // model.eval_tableau();
    if let Some(active) = model.tableau.active_nodes() {
        for n in active.iter() {
            println!("{n}");
        }
    }
    println!("Run to the end");

    Ok(())
}
