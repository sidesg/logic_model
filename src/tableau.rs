use std::collections::{HashMap, HashSet};
use crate::searches::{GraphSearch, BreadthFirstSearch};

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Tableau {
    nodes: HashMap<usize, Node>,
    adj: HashMap<usize, HashSet<usize>>
}

impl Tableau {
    pub fn new(formulas: Vec<String>) -> Tableau {
        fn next_child(node: usize, len: usize) -> HashSet<usize> {
            let mut output = HashSet::new();
            let child = node + 1;
            if child < len {
                output.insert(child);
            } 
            
            output
        }
        let node_count = formulas.len();

        Tableau {
            nodes: formulas.into_iter().enumerate()
                .map(|(idx, formula)| (idx, Node::new(formula, 0)))
                .collect::<HashMap<usize, Node>>(),
            adj: (0..node_count).into_iter()
                .map(|idx| (idx, next_child(idx, node_count)))
                .collect::<HashMap<usize, HashSet<usize>>>()
        }
    }

    fn testing_new(formulas: Vec<String>) -> Tableau {
        let node_count = formulas.len();

        Tableau {
            nodes: formulas.into_iter().enumerate()
                .map(|(idx, formula)| (idx, Node::new(formula, 0)))
                .collect::<HashMap<usize, Node>>(),
            adj: (0..node_count).into_iter()
                .map(|idx| (idx, HashSet::new()))
                .collect::<HashMap<usize, HashSet<usize>>>()
        }
    }

    pub fn active_nodes(&self) -> Option<Vec<usize>> {
        let active_nodes = self.nodes.iter()
            .filter(|(_, node)| node.state == NodeState::Active)
            .map(|(idx, _)| *idx)
            .collect::<Vec<usize>>();

        if active_nodes.len() == 0 {
            None
        } else {
            Some(active_nodes)
        }
    }

    pub fn first_active_node(&self) -> Option<usize> {
        self.active_nodes()?
            .iter()
            .next()
            .copied()
    }

    pub fn add_child(&mut self, from_node: usize, to_node: usize) {
        if let Some(adjacencies) = self.adj.get_mut(&from_node) {
            adjacencies.insert(to_node);
        }
    }

    pub fn unclosed_branches(&self) -> Option<Vec<Vec<usize>>> {
        let paths = self.terminal_unclosed(0)?.iter()
            .map(|t_node| BreadthFirstSearch::shortest_path(self, 0, *t_node).unwrap())
            .collect();
        Some(paths)
    }

    fn terminal_unclosed(&self, root: usize) -> Option<Vec<usize>> {
        let terminal_unclosed = self.active_nodes()?.iter()
            .filter(|idx| self.adj(**idx).unwrap().len() == 0)
            .map(|idx| *idx)
            .collect(); 
        Some(terminal_unclosed)
    }
}

impl GraphSearch for Tableau {
    fn adj(&self, v: usize) -> Option<HashSet<usize>> {
        self.adj.get(&v).cloned()
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Node {
    formula: String,
    world: usize,
    state: NodeState,
}

impl Node {
    fn new(formula: String, world: usize) -> Node {
        Node {
            formula: formula,
            world: world,
            state: NodeState::Active
        }
    }

    pub fn formula(&self) -> &String {
        &self.formula
    }

    pub fn deactivate(&mut self) {
        self.state = NodeState::Inactive;
    }

    pub fn wait(&mut self) {
        self.state = NodeState::WaitingNewWorlds;
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum NodeState {
    Active,
    Inactive,
    WaitingNewWorlds,
    Closed,
}

#[test]
fn active_tests() {
    let rootformulas: Vec<String> = vec![
        String::from("first formula"),
        String::from("second formula"),
        String::from("third formula")
    ];
    let mut tableau = Tableau::new(rootformulas);

    assert_eq!(3, tableau.active_nodes().unwrap().len());

    for node in tableau.nodes.values_mut() {
        node.deactivate();
    };

    assert_eq!(None, tableau.active_nodes())

}

#[test]
fn new_tableau() {
    let rootformulas: Vec<String> = vec![
        String::from("first formula"),
        String::from("second formula"),
        String::from("third formula")
    ];
    let tab = Tableau::new(rootformulas);

    let mut hash_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut map: HashSet<usize> = HashSet::new();
    map.insert(1);
    hash_map.insert(0, map);
    let mut map: HashSet<usize> = HashSet::new();
    map.insert(2);
    hash_map.insert(1, map);
    hash_map.insert(2, HashSet::new());

    assert_eq!(tab.adj, hash_map)
}

#[test]
fn branching_test() {
    let formulas = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string()
    ];
    let mut tableau = Tableau::testing_new(formulas);

    tableau.add_child(0, 1);
    tableau.add_child(1, 2);
    tableau.add_child(2, 3);
    tableau.add_child(2, 4);
    tableau.add_child(3, 5);
    tableau.add_child(4, 6);

    let mut terminals = tableau.unclosed_branches().unwrap();
    terminals.sort();

    let ex_vecs: Vec<Vec<usize>> = vec![
        vec![5, 3, 2, 1, 0],
        vec![6, 4, 2, 1, 0]
    ];

    assert_eq!(ex_vecs, terminals)
}
