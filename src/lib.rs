#![allow(dead_code)]
#![allow(unused_variables)]

mod world;
use std::io::Error;
use std::fs::read_to_string;

use crate::world::{World, WorldGraph};
mod tableau;
use crate::tableau::Tableau;
mod parser;
use crate::parser::{Parser, InstructionOperator, Instructions};
mod searches;
mod modal_config;
use crate::modal_config::ModalOptions;


pub struct Model {
    worlds: WorldGraph,
    modal_options: ModalOptions,
    tableau: Tableau
}

impl Model {
    pub  fn new(options: ModalOptions, formulas: Vec<String>) -> Model {
        Model {
            worlds: WorldGraph::new(1),
            modal_options: options,
            tableau: Tableau::new(formulas)
        }
    }

    pub fn from_file(filename: &str) -> Result<Model, Error> {
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

#[test]
fn test_test() {
    let rootformulas: Vec<String> = vec![
        String::from("first formula"),
        String::from("second formula"),
        String::from("third formula")
    ];
    let model = Model::new(ModalOptions::all_true(), rootformulas);
}
