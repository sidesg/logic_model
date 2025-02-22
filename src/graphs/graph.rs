use std::collections::HashSet;
use crate::graphs::{Formula, FormulaState, World};
use crate::graphs::search::GraphSearch;
use crate::modal_config::ModalOptions;

pub struct Graph<T> {
    adjacencies: Vec<HashSet<usize>>,
    nodes: Vec<T>
}

impl<T> Graph<T> {
    pub fn adj_to(&self, n: usize) -> Option<HashSet<usize>> {
        self.adjacencies.iter().nth(n).cloned()
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        if let Some(neighbours) = self.adjacencies.iter_mut().nth(v) {
            if !neighbours.contains(&w) {
                neighbours.insert(w);
            }
        }
    }

    pub fn adj_test(&self, u: usize, w: usize) -> Option<bool> {
        Some(self.adjacencies.iter().nth(u)?
            .contains(&w))
    }

    pub fn node_ids(&self) -> Vec<usize> {
        (0..self.size()).collect()
    }

    pub fn get_node_mut(&mut self, id: usize) -> Option<&mut T> {
        self.nodes.iter_mut().nth(id)
    }

    pub fn get_node(&self, id: usize) -> Option<&T> {
        self.nodes.iter().nth(id)
    }
}

impl Graph<World> {
    pub fn new(n: usize) -> Graph<World> {
        let mut adjacencies: Vec<HashSet<usize>> = Vec::new(); 
        let mut nodes: Vec<World> = Vec::with_capacity(n);
        for i in 0..n {
            adjacencies.push(HashSet::new());
            nodes.push(World::new(i));
        };

        Graph {
            adjacencies,
            nodes
        }
    }

    pub fn add_world(&mut self) -> usize {
        let new_idx = self.size() - 1;
        self.nodes.push(World::new(new_idx));
        self.adjacencies.push(HashSet::new());
        new_idx
    }

    pub fn implement_modals(&mut self, config: &ModalOptions) {
        if config.rho() {
            // reflexive
            for id in self.node_ids() {
                self.add_edge(id, id);
            }
        }
        if config.tau() {
            // transitive
            let worlds: Vec<usize> = self.node_ids();
            for w in worlds {
                if let Some(bfs) = GraphSearch::bfs(self, w).all_marked() {
                    for w_prime in bfs { self.add_edge(w, w_prime); }
                }
            }
        }
        if config.sigma() {
            // symmetrical 
            let nodes: Vec<usize> = self.node_ids();
            for w in nodes {
                let adjs = self.adj_to(w).unwrap();
                for w_prime in adjs {
                    self.add_edge(w_prime, w);
                }
            }
        }
        if config.eta() {
            // extendable
            todo!()
        }
    }  
}

impl Graph<Formula> {
    pub fn new(formulas: Vec<String>) -> Graph<Formula> {
        fn next_child(node: usize, len: usize) -> HashSet<usize> {
            let mut output = HashSet::new();
            let child = node + 1;
            if child < len {
                output.insert(child);
            }

            output
        }
        let node_count = formulas.len();

        Graph {
            nodes: formulas.into_iter()
                .map(|formula| Formula::new(formula, 0))
                .collect::<Vec<Formula>>(),
            adjacencies: (0..node_count)
                .map(|idx| next_child(idx, node_count))
                .collect::<Vec<HashSet<usize>>>()
        }
    }

    pub fn active_nodes(&self) -> Option<Vec<usize>> {
        let active_nodes = self.nodes.iter().enumerate()
            .filter(|(_, node)| *node.state() == FormulaState::Active)
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();

        if active_nodes.is_empty() {
            None
        } else {
            Some(active_nodes)
        }
    }

    pub fn first_active_node(&self) -> Option<usize> {
        self.active_nodes()?
            .first()
            .copied()
    }

    pub fn terminal_unclosed(&self, root: usize) -> Option<Vec<usize>> {
        let search = GraphSearch::bfs(self, root);
        let terminal_unclosed = search.all_marked()?.iter()
            .filter(|idx| self.adj_to(**idx).unwrap().is_empty())
            .filter(|idx| self.get_node(**idx).unwrap().state() != &FormulaState::Closed)
            .copied()
            .collect();
        Some(terminal_unclosed)
    }

    pub fn unclosed_branches(&self) -> Option<Vec<Vec<usize>>> {
        let paths = self.terminal_unclosed(0)?.iter()
            .map(|t_node| GraphSearch::shortest_path(self, 0, *t_node).unwrap())
            .collect();
        Some(paths)
    }

    pub fn find_contradictions(&mut self) {
        // for each unclosed branch, look for pairs of nodes p, ¬p
        // if found, close terminal node
        if let Some(unclosed_branches) = self.unclosed_branches() {
            for branch in unclosed_branches {
                for node in branch.iter() {
                    for node_prime in &branch[*node..] {
                        todo!()
                        // compare node and node_prime
                    }
                }
            }
        }
    }
    pub fn new_node_from(&mut self, parent: usize, formula: String, world: usize) {
        let new_node = Formula::new(formula, world);
        self.nodes.push(new_node);
        self.add_edge(parent, self.size() - 1);
        self.adjacencies.push(HashSet::new());
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use  super::*;

    #[test]
    fn worlds() {
        let mut worlds = Graph::<World>::new(4);

        assert_eq!(4, worlds.size());
        assert_eq!(vec![0,1,2,3], worlds.node_ids());

        worlds.add_edge(0, 1);
        assert_eq!(true, worlds.adj_test(0, 1).unwrap());
        assert_eq!(false, worlds.adj_test(0, 2).unwrap());

        worlds.add_world();
        assert_eq!(5, worlds.size());
        assert_eq!(vec![0,1,2,3,4], worlds.node_ids());
        
    }

    #[test]
    fn modal_connections() {
        let config = ModalOptions::new_default();
        let mut worlds = Graph::<World>::new(10);

        let ids = worlds.node_ids();
        for id in ids.iter() {
            if ids.contains(&(id + 1)) {
                worlds.add_edge(*id, id+1);
            }
        }
        worlds.implement_modals(&config);
        // every node connected to every other
        for id in ids.iter() {
            for next in &ids[*id..] {
                println!("{} {}", id, next);
                assert_eq!(true, worlds.adj_test(*id, *next).unwrap());
            }
        }
    }

    #[test]
    fn formalas() {
        let world: usize = 0;
        let formulas = vec![
            "p then q".to_owned(),
            "p".to_owned(),
            "not q".to_owned()
        ];

        // set up downward branching graph
        let mut graph = Graph::<Formula>::new(formulas);
        // new node from
        graph.new_node_from(2, "not p".to_owned(), 0);
        graph.new_node_from(2, "q".to_owned(), 0);   
        // deactivate some nodes
        graph.get_node_mut(0).unwrap().deactivate();
        graph.get_node_mut(1).unwrap().deactivate();
        // graph.get_node_mut(2).unwrap().deactivate();

        // find active
        let mut actives = graph.active_nodes().unwrap();
        actives.sort();
        assert_eq!(vec![2,3,4], actives);

        // first active
        let firsta = graph.first_active_node().unwrap();
        assert_eq!(2, firsta);

        assert_eq!(vec![1_usize], graph.adj_to(0).unwrap().into_iter().collect::<Vec<usize>>());
        assert_eq!(HashSet::new(), graph.adj_to(3).unwrap());

        graph.get_node_mut(4).unwrap().close();
        // unclosed branches
        let test_branch: Vec<Vec<usize>> = vec![vec![0,1,2,3]];
        assert_eq!(test_branch, graph.unclosed_branches().unwrap());

        // terminal unclosed
        assert_eq!(vec![3], graph.terminal_unclosed(0).unwrap());

    }
}
