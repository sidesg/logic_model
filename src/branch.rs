use std::{f32::consts::E, rc::Rc};    
use crate::world::World;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Branch {
    formulas: Vec<Node>,
    children: Option<Vec<Box<Branch>>>,
    state: BranchState
}

#[derive(PartialEq)]
#[derive(Debug)]
enum BranchState {
    Open,
    Closed
}

impl Branch {
    pub fn make_root(formulas: Vec<String>, world0: Rc::<World>) -> Branch {
        Branch {
            formulas: formulas
                .into_iter()
                .map(|exp| Node::new(exp, Rc::clone(&world0)))
                .collect(),
            children: None,
            state: BranchState::Open
        }
    }

    pub fn next_active_node(&mut self) -> Option<&mut Node> {
        let fist_node = Branch::get_active_in_branch(&mut self.formulas);
        fist_node
    }

    fn get_active_in_branch(formulas: &mut Vec<Node>) -> Option<&mut Node> {
        formulas.iter_mut()
            .filter(|node| node.state == NodeState::Active)
            .next()
    }

    pub fn split_branch(&self) {
        todo!()
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum NodeState {
    Active,
    Inactive
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Node {
    formula: String,
    world: Rc<World>,
    state: NodeState
}

impl Node {
    fn new(formula: String, world: Rc<World>) -> Node {
        Node {
            formula: formula,
            world: world,
            state: NodeState::Active
        }
    }

    fn deactivate(&mut self) {
        self.state = NodeState::Inactive;
    }
}
