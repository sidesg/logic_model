use std::collections::{HashMap, HashSet};
use crate::searches::{GraphSearcher, GraphSearch};
use crate::modal_config::ModalOptions;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct World {
    id: usize,
    c: Option<String>
}

impl World {
    pub fn new(id: usize) -> World {
        World{
            id,
            c: None
        }
    }
}

pub struct WorldGraph {
    v: usize,
    e: usize,
    adj: HashMap<usize, HashSet<usize>>,
    worlds: HashMap<usize, World>
}

impl WorldGraph {
    pub fn new(v: usize) -> WorldGraph {
        let mut adj: HashMap<usize, HashSet<usize>> = HashMap::new(); 
        let mut worlds: HashMap<usize, World> = HashMap::with_capacity(v);
        for i in 0..v {
            adj.insert(i, HashSet::new());
            worlds.insert(
                i,
                World{id: i, c: None}
            );
        };
        WorldGraph{
            v,
            e: 0,
            adj,
            worlds
        }
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn all_worlds(&self) -> Vec<usize> {
        self.adj.iter().map(|(k, v)| *k).collect()
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        if let Some(children) = self.adj.get_mut(&v) {
            if children.get(&w).is_none() {
                children.insert(w);
                self.e += 1;
            }
        }
    }

    fn next_world(&self) -> usize {
        if let Some(next) = self.adj.keys().max() {
                next + 1
        } else {
            0
        }
    }

    pub fn connect_all(&mut self) {
        let mut worldids: Vec<usize> = Vec::new();
        let _ = self.worlds.keys().map(|k| worldids.push(*k));

        for source in worldids.iter() {
            for target in worldids.iter() {
                self.add_edge(*source, *target);
            }        
        }
    }

    pub fn accessible(&self, node_id: usize) -> Option<Vec<&World>> {
        // if let Some(accessible_words) = self.adj(node_id) {
        //     Some(accessible_words.iter()
        //         .map(|i| self.worlds.get(i).unwrap())
        //         .collect::<Vec<&World>>()
        //     )
        // } else {
        //     None
        // }
        self.adj(node_id).map(|accessible_worlds| accessible_worlds.iter()
            .map(|i| self.worlds.get(i).unwrap())
            .collect::<Vec<&World>>())
    }

    pub fn add_world(&mut self) -> usize {
        let new_idx = self.next_world();
        self.worlds.insert(new_idx, World { id: new_idx, c: None });
        self.adj.insert(new_idx, HashSet::new());
        new_idx
    }

    pub fn implement_modals(&mut self, config: &ModalOptions) {
        if config.rho() {
            // reflexive
            let _ = self.adj.iter_mut()
                        .map(|(k, v)| v.insert(*k));
        }
        if config.sigma() {
            // symmetrical 
            let nodes: Vec<usize> = self.all_worlds();
            for w in nodes {
                let adjs = self.adj(w).unwrap().clone();
                for w_prime in adjs {
                    self.add_edge(w_prime, w);
                }
            }
        }
        if config.tau() {
            // transitive
            let worlds = self.all_worlds();
            for w in worlds {
                if let Some(bfs) = GraphSearch::bfs(self, w).all_marked() {
                    for w_prime in bfs { self.add_edge(w, w_prime); }
                }
            }
        }
        if config.eta() {
            // extendable
            todo!()
        }
    }
}

impl GraphSearcher for WorldGraph {
    fn adj(&self, v: usize) -> Option<HashSet<usize>> {
        self.adj.get(&v).cloned()
    }

    fn v(&self) -> usize {
        self.worlds.len()
    }
}

#[test]
fn make_graph() {
    let wg = WorldGraph::new(10);

    assert_eq!(wg.adj.len(), wg.v.try_into().unwrap())
}

#[test]
fn world0() {
    let wg = WorldGraph::new(1);
    let w0 = wg.worlds.get(&0).unwrap();

    assert_eq!(0, w0.id)
}

#[test]
fn add_edge() {
    let mut wg = WorldGraph::new(10);
    wg.add_edge(1, 3);

    assert_eq!(1, wg.e.try_into().unwrap()) 
}

#[test]
fn next_world() {
    let wg = WorldGraph::new(10);
    let next = wg.next_world();

    assert_eq!(10, next)
}

#[test]
fn adjacency() {
    let mut wg = WorldGraph::new(10);
    wg.add_edge(1, 2);
    wg.add_edge(2, 3);
    wg.add_edge(2, 4);
    wg.add_edge(3, 5);

    let mut adj: Vec<usize> = wg.adj(2)
        .unwrap()
        .into_iter()
        .collect(); 
    adj.sort();

    assert_eq!(vec![3, 4], adj)
}

#[test]
fn basic_search() {
    let mut wg = WorldGraph::new(10);
    wg.add_edge(1, 2);
    wg.add_edge(2, 3);
    wg.add_edge(2, 4);
    wg.add_edge(3, 5);

    let search = GraphSearch::dfs(&wg, 2);
    let mut available = search.all_marked().unwrap();
    available.sort();

    assert_eq!(available.len(), 3);
    assert_eq!(available, vec![3, 4, 5]);

    assert_eq!(search.has_path_to(1), false);
    assert_eq!(search.has_path_to(4), true);
    assert_eq!(search.has_path_to(2), false);

    assert_eq!(search.path_to(1), None);

    let path = search.path_to(5).unwrap(); 
    assert_eq!(vec![5, 3, 2], path);
}

#[test]
fn shortest_path() {
    let mut wg = WorldGraph::new(10);
    wg.add_edge(1, 2);
    wg.add_edge(2, 3);
    wg.add_edge(2, 4);
    wg.add_edge(3, 5);
    wg.add_edge(4, 6);
    wg.add_edge(5, 6);

    let shortest_path = GraphSearch::shortest_path(&wg, 1, 6);
    assert_eq!(vec![6, 4, 2, 1], shortest_path.unwrap());
}
