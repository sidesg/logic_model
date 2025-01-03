use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
#[derive(Debug)]
pub struct World {
    id: usize,
    c: Option<String>
}

impl World {
    pub fn new(id: usize) -> World {
        World{
            id: id,
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

// todo: turn graph behaviour into trait to generalize graph
impl WorldGraph {
    pub fn new(v: usize) -> WorldGraph {
        let mut adj: HashMap<usize, HashSet<usize>> = HashMap::new(); 
        let mut worlds: HashMap<usize, World> = HashMap::new();
        for i in 0..v {
            adj.insert(i, HashSet::new());
            worlds.insert(
                i,
                World{id: i, c: None}
            );
        };
        WorldGraph{
            v: v,
            e: 0,
            adj: adj,
            worlds: worlds
        }
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        if let Some(children) = self.adj.get_mut(&v) {
            if children.get(&w).is_none() {
                children.insert(w);
                self.e += 1;
            }
        }
    }

    pub fn adj(&self, v: usize) -> Option<HashSet<usize>> {
        self.adj.get(&v).cloned()
    }

    fn next_world(&self) -> usize {
        let next = self.adj
            .keys()
            .max()
            .unwrap_or(&0);

        next + 1
    }

    pub fn connect_all(&mut self) {
        let mut worldids: Vec<usize> = Vec::new();
        let _ = self.worlds.iter().map(|(k, _)| worldids.push(*k));

        for source in worldids.iter() {
            for target in worldids.iter() {
                self.add_edge(*source, *target);
            }        
        }
    }

    pub fn accessible(&self, node_id: usize) -> Vec<&World> {
        let mut search = DepthFirstSearch::new();
        let worlds: Vec<&World> = search
            .search(self, node_id)
            .iter()
            .map(|i| self.worlds.get(i).unwrap())
            .collect();
        worlds
    }

    pub fn add_world(&mut self) -> usize {
        let new_idx = self.next_world();
        self.worlds.insert(new_idx, World { id: new_idx, c: None });
        self.adj.insert(new_idx, HashSet::new());
        new_idx
    }
}

struct DepthFirstSearch {
    marked: Vec<usize>,
    count: usize
}

impl DepthFirstSearch {
    pub fn new() -> DepthFirstSearch {
        DepthFirstSearch { marked: Vec::new(), count: 0 }
    }
    pub fn search(&mut self, g: &WorldGraph, source: usize) -> Vec<usize> {
        if let Some(worlds) = g.adj(source) {
            for v in worlds.iter() {
                self.dfs(g, *v);
            }
        }
        self.marked.clone() 
    }

    fn dfs(&mut self, g: &WorldGraph, vertex: usize) {
        self.marked.push(vertex);
        self.count += 1;

        if let Some(adj) = g.adj(vertex) {
            for w in adj.iter() {
                if !(self.marked.contains(&w)) {
                    self.dfs(g, *w);
                }
            }
        }
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

    let adj: Vec<usize> = wg.adj(2)
        .unwrap()
        .into_iter()
        .collect();

    assert_eq!(vec![3, 4], adj)
}

#[test]
fn basic_search() {
    let mut wg = WorldGraph::new(10);
    wg.add_edge(1, 2);
    wg.add_edge(2, 3);
    wg.add_edge(2, 4);
    wg.add_edge(3, 5);

    let mut searcher = DepthFirstSearch::new();
    searcher.search(&wg, 2);

    assert_eq!(searcher.marked.len(), 3);
    assert_eq!(searcher.marked, vec![3, 4, 5])
}
