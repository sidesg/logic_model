use std::fs::read_to_string;
use std::process::exit;
use crate::world::WorldGraph;
use crate::tableau::Tableau;
use crate::modal_config::ModalOptions;
use crate::parser::{Instructions, parse_formula};

pub struct Model {
    worlds: WorldGraph,
    modal_options: ModalOptions,
    pub tableau: Tableau
}

impl Model {
    pub  fn new(options: ModalOptions, formulas: Vec<String>) -> Model {
        Model {
            worlds: WorldGraph::new(1),
            modal_options: options,
            tableau: Tableau::new(formulas)
        }
    }

    pub fn from_file(filename: &str) -> Result<Model, Box< std::io::Error>> {
        let formulas: Vec<String> = read_to_string(filename)?
            .lines()
            .map(String::from)
            .collect();

        let model = Model {
            worlds: WorldGraph::new(1),
            modal_options: ModalOptions::new_default(),
            tableau: Tableau::new(formulas.clone())
        };

        tracing::info!("Model built {}", formulas.join(", "));
        Ok(model)
    }

    pub fn eval_tableau(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(node_id) = self.tableau.first_active_node() {
            self.eval_node(node_id)?;
            self.tableau.find_contradictions();
            if self.tableau.unclosed_branches().is_some() { todo!() };

            // update wrw
            // apply waiting necessity formulae
            self.tableau.find_contradictions();
            if self.tableau.unclosed_branches().is_some() { todo!() };
        }

        if let Some(open_branches) = self.tableau.unclosed_branches() {
            let branch = open_branches.first().unwrap().clone();
            self.build_countermodel(branch);
        }

        Ok(())
    }

    fn eval_node(&mut self, node_id: usize) -> Result<(), Box<dyn std::error::Error>> {
        let node = self.tableau.get_node(node_id)
            .expect("Calling function should make sure node_id is valid");
        let instructions: Instructions = parse_formula(node.formula())?;
        self.implement_instructions(instructions, node_id);
        // implement instructions
        //      create worlds, wrw
        //      add formulae to open terminals
        Ok(())
    }

    // move implementation details to own file
    fn implement_instructions(&mut self, instructions: Instructions, node_id: usize) {
        match instructions.operators().as_str() {
            "⋀" => todo!(),
            "⋁" => todo!(),
            "⊃" => {
                if instructions.variables().len() > 2 {
                    tracing::warn!("Too many variables in {}: {}. Ignoring extra.", instructions.operators(), instructions.variables().join(", "));
                };
                if instructions.variables().len() < 2 {
                    tracing::error!("Unable to parse {}: {}. Too few variables", instructions.operators(), instructions.variables().join(", "));
                    exit(1);
                }
                let ant = "¬".to_string() + instructions.variables().first().unwrap();
                let cons = instructions.variables().first().unwrap();
                // get all terminals relative to node_id
                if let Some(terminals) = self.tableau.terminal_unclosed(node_id) {
                    for terminal in terminals {
                        let parent_world = self.tableau.get_node(node_id)
                            .unwrap()
                            .world();
                        self.tableau.new_node_from(node_id, ant.clone(), parent_world);
                        self.tableau.new_node_from(node_id, cons.clone(), parent_world);
                    }
                }; // do something if no terminal unclosed?
            },
            "¬" => todo!(),
            "◻" => todo!(),
            "◇" => todo!(),
            "" => todo!(),
            _ => {
                tracing::error!("No instructions found for {}", instructions.operators());
                exit(1);
            }
        }
    }

    fn build_countermodel(&self, branch: Vec<usize>) {
        todo!()
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file() {
        let model = Model::from_file("data/basic.txt").unwrap();
        assert_eq!(3, model.tableau.size());
    }

    #[test]
    #[should_panic]
    fn from_file_err() {
        let model = Model::from_file("adfasdfa").unwrap();
    }
}
