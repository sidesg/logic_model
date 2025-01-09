#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::rc::Rc;    

mod world;
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
    wrw: Option<Vec<(World, World)>>,
    tableau: Tableau
}

impl Model {
    pub  fn new(options: ModalOptions, formulas: Vec<String>) -> Model {
        let world0 = Rc::new(World::new(0));
        let mut worlds = HashMap::new();
        worlds.insert(0, Rc::clone(&world0));
        
        Model {
            worlds: WorldGraph::new(1),
            modal_options: options,
            wrw: None,
            tableau: Tableau::new(formulas)
        }
    }

    
    // pub fn evaluate_next_node(&mut self) {
    //     let active_node = self.tableau.get_first_active_node();
    //     match self.tableau.get_first_active_node() {
    //         Some(active_node) => {
    //             let instructions = Parser::parse_formula(&active_node.formula).unwrap();
    //             self.implement_instructions(instructions);
    //             // update R
    //             // update necessity formulas
    //             // add closures
    //             // check terminals {if all closed, entailment obtains}
    //         },
    //         None => {
    //             match self.tableau.get_unclosed() {
    //                 Some(unclosed_branches) => self.build_countermodel(),
    //                 None => todo!() // entailment obtains
    //             }
    //         }
    //     }
    // }

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
    
    fn add_world(&mut self, from: World) {
        todo!()
    }

    fn update_wrw(&mut self) {
        todo!()
    }

    fn build_countermodel(&self) {
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
