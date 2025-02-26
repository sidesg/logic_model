use::std::collections::{HashMap, VecDeque};
use crate::graphs::Graph;

pub struct GraphSearch {
    source: usize,
    marked: Vec<usize>,
    edge_to: Option<HashMap<usize, usize>>,
}

impl GraphSearch {
    pub fn dfs<T>(g: &Graph<T>, source: usize) -> GraphSearch {
        let mut dfs = GraphSearch {
            source,
            marked: Vec::new(),
            edge_to: Some(HashMap::with_capacity(g.size()))
        };

        if let Some(worlds) = g.adj_to(source) {
            dfs.inner_dfs(g, source);
        } else {
            dfs.edge_to = None;
        }
        dfs
    }

    fn inner_dfs<T>(&mut self, g: &Graph<T>, v:usize) {
        if let Some(adj) = g.adj_to(v) {
            for w in adj.iter() {
                if !(self.marked.contains(w)) {
                    self.edge_to.as_mut()
                        .expect("New search should always start with Some(Hashmap)")
                        .insert(*w, v);
                    self.marked.push(*w);
                    self.inner_dfs(g, *w);
                }
            }
        }       
    }

    pub fn bfs<T>(g: &Graph<T>, source: usize) -> GraphSearch {
        let mut bfs = GraphSearch{
            source,
            marked: Vec::new(),
            edge_to: Some(HashMap::with_capacity(g.size()))
        };
        
        if let Some(worlds) = g.adj_to(source) {
            bfs.inner_bfs(g, source);
        } else {
            bfs.edge_to = None;
        }

        bfs
    }

    fn inner_bfs<T>(&mut self, g: &Graph<T>, source: usize) {
        let mut queue: VecDeque<usize> = VecDeque::new();
        // self.marked.push(source);
        queue.push_back(source);

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            for w in g.adj_to(v).unwrap().iter() {
                if !(self.marked.contains(w)) {
                    self.edge_to.as_mut()
                        .expect("New seach should always have Some(HashMap)")
                        .insert(*w, v);
                    self.marked.push(*w);
                    queue.push_back(*w);
                }
            }
        }
    }

    pub fn all_marked(&self) -> Option<Vec<usize>> {
        if self.marked.is_empty() {
            None
        } else {
            Some(self.marked.clone())
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked.contains(&v)
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if !self.has_path_to(v) {
            None
        } else {
            let mut path: Vec<usize> = Vec::new();
            path.push(v);

            let mut x = v;
            loop {
                x = *(self.edge_to.as_ref()
                    .expect("If edge_to is None, flow should follow first if")
                    .get(&x).unwrap());
                if x == self.source { break };
                path.push(x);    
            }
            path.push(self.source);
            path.sort();
            Some(path)
        }
    }

    pub fn shortest_path<T>(g: &Graph<T>, source: usize, target: usize) -> Option<Vec<usize>> {
        let mut bfs = GraphSearch{
            source,
            marked: Vec::new(),
            edge_to: Some(HashMap::with_capacity(g.size()))
        };
        let mut queue: VecDeque<usize> = VecDeque::new();
        bfs.marked.push(source);
        queue.push_back(source);

        'outer: while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            for w in g.adj_to(v).unwrap().iter() {
                if *w == target {
                    bfs.edge_to.as_mut()
                        .expect("New search always has Some(Vec)")
                        .insert(*w, v);
                    bfs.marked.push(*w);
                    break 'outer;                    
                } else if !(bfs.marked.contains(w)) {
                    bfs.edge_to.as_mut()
                        .expect("New search always has Some(Vec)")
                        .insert(*w, v);
                    bfs.marked.push(*w);
                    queue.push_back(*w);
                }
            }
        }
        bfs.path_to(target)
    }
}
