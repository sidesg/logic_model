#![allow(dead_code)]
#![allow(unused_variables)]

mod world;
use std::fs::read_to_string;

use crate::world::WorldGraph;
mod tableau;
use crate::tableau::Tableau;
mod parser;
use crate::parser::{InstructionOperator, Instructions};
mod searches;
mod modal_config;
use crate::modal_config::ModalOptions;
pub mod configs;

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

    pub fn from_file(filename: &str) -> Result<Model, std::io::Error> {
        let formulas: Vec<String> = read_to_string(filename)?
            .lines()
            .map(String::from)
            .collect();

        let model = Model {
            worlds: WorldGraph::new(1),
            modal_options: ModalOptions::new_default(),
            tableau: Tableau::new(formulas)
        };

        Ok(model)
    }

    pub fn eval_tableau(&mut self) {
        while let Some(node_id) = self.tableau.first_active_node() {
            self.eval_node(node_id);
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
    }

    fn eval_node(&mut self, node_id: usize) {
        let node = self.tableau.get_node(node_id)
            .expect("Calling function should make sure node_id is valid");
        // get instructions
        // implement instructions
        //      create worlds, wrw
        //      add formulae to open terminals 
    }

    fn implement_instructions(&mut self, instructions: Instructions) {
        match instructions.operator {
            InstructionOperator::And => todo!(),
            InstructionOperator::Or => todo!(),
            InstructionOperator::MaterialImplecation => todo!(),
            InstructionOperator::Not => todo!(),
            InstructionOperator::Necessary => todo!(),
            InstructionOperator::Possible => todo!()
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
