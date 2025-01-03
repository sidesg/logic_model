use std::collections::HashMap;
use std::rc::Rc;
use crate::world::World;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Tableau {
    nodes: Vec<Node>,
    relations: HashMap<u64, Vec<u64>>
}

impl Tableau {
    pub fn new(formulas: Vec<String>, world0: &Rc::<World>) -> Tableau {
        Tableau {
            nodes: formulas.into_iter()
                .map(|formula| Node::new(formula, Rc::clone(world0)))
                .collect(),
            relations: HashMap::new()
        }
    }

    pub fn get_active_nodes(&mut self) -> Option<Vec<&mut Node>> {
        let active_nodes = self.nodes
            .iter_mut()
            .filter(|node| node.state == NodeState::Active)
            .collect::<Vec<&mut Node>>();

        if active_nodes.len() == 0 {
            None
        } else {
            Some(active_nodes)
        }
    }

    pub fn get_first_active_node(&mut self) -> Option<&mut Node> {
        if let Some(first_active) = self.nodes
            .iter_mut()
            .filter(|node| node.state == NodeState::Active)
            .next() { 
                Some(first_active)
        } else {
            None
        }
    }

    pub fn get_unclosed(&self) -> Option<Vec<&Node>> {
        todo!()
    }

    pub fn get_terminal_unclosed(&self) -> Vec<&Node> {
        // use this to find where to append new formulae wrt a specific node
        todo!()
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum NodeState {
    Active,
    Inactive,
    WaitingNewWorlds,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Node {
    pub formula: String,
    world: Rc<World>,
    state: NodeState,
}

impl Node {
    fn new(formula: String, world: Rc<World>) -> Node {
        Node {
            formula: formula,
            world: world,
            state: NodeState::Active
        }
    }

    pub fn deactivate(&mut self) {
        self.state = NodeState::Inactive;
    }

    pub fn wait(&mut self) {
        self.state = NodeState::WaitingNewWorlds;
    }
}

#[test]
fn active_tests() {
    let w0 = Rc::new(World::new(0));
    let rootformulas: Vec<String> = vec![
        String::from("first formula"),
        String::from("second formula"),
        String::from("third formula")
    ];
    let mut tableau = Tableau::new(rootformulas, &w0);

    assert_eq!(3, tableau.get_active_nodes().unwrap().len());

    for node in tableau.nodes.iter_mut() {
        node.deactivate();
    };

    assert_eq!(None, tableau.get_active_nodes())

}
