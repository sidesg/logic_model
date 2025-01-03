#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::rc::Rc;    

mod world;
use parser::Instructions;

use crate::world::World;

mod branch;
use crate::branch::Branch;

mod parser;
use crate::parser::{Parser, InstructionOperator};

struct ModalOptions {
    rho: bool,
    sigma: bool,
    tau: bool,
    eta: bool
}

impl ModalOptions {
    pub fn all_true() -> ModalOptions {
        ModalOptions {
            rho: true,
            sigma: true,
            tau: true,
            eta: true
        }
    }
}

struct Model {
    worlds: HashMap<u64, Rc<World>>,
    modal_options: ModalOptions,
    wrw: Option<Vec<(World, World)>>,
    tableau: Branch
}

impl Model {
    pub  fn new(options: ModalOptions, formulas: Vec<String>) -> Model {
        let world0 = Rc::new(World::first_world());
        let mut worlds = HashMap::new();
        worlds.insert(0, Rc::clone(&world0));
        
        Model {
            worlds: worlds,
            modal_options: options,
            wrw: None,
            tableau: Branch::make_root(formulas, &world0)
        }
    }

    pub fn evaluate_next_node(&mut self) {
        let active_node = self.tableau.get_first_active_node();
        match self.tableau.get_first_active_node() {
            Some(active_node) => {
                let instructions = Parser::parse_formula(&active_node.formula).unwrap();
                self.implement_instructions(instructions);
            },
            None => {
                match self.tableau.get_unclosed() {
                    Some(unclosed_branches) => self.build_countermodel(),
                    None => todo!() // entailment obtains
                }
            }
        }
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
