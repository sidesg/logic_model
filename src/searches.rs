use::std::collections::{HashSet, HashMap, VecDeque};

pub trait GraphSearch {
    fn adj(&self, v: usize) -> Option<HashSet<usize>>;
}

pub struct DepthFirstSearch {
    marked: Vec<usize>,
    edge_to: HashMap<usize, usize>,
    count: usize,
    source: usize
}

impl DepthFirstSearch {
    pub fn new_search<T: GraphSearch>(g: &T, source: usize) -> DepthFirstSearch {
        let mut dfs = DepthFirstSearch {
            source: source,
            count: 0,   
            marked: Vec::new(),
            edge_to: HashMap::new()
        };

        if let Some(worlds) = g.adj(source) {
            dfs.dfs(g, source);
        }
        dfs
    }

    fn dfs<T: GraphSearch>(&mut self, g: &T, vertex: usize) {
        if let Some(adj) = g.adj(vertex) {
            for w in adj.iter() {
                if !(self.marked.contains(&w)) {
                    self.edge_to.insert(*w, vertex);
                    self.dfs(g, *w);
                    self.marked.push(*w);
                    self.count += 1;
                }
            }
        }
    }
    
    pub fn all_marked(&self) -> Option<Vec<usize>> {
        if self.marked.len() == 0 {
            None
        } else {
            Some(self.marked.clone())
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked.contains(&v)
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if self.has_path_to(v) == false {
            None
        } else {
            let mut path: Vec<usize> = Vec::new();
            path.push(v);

            let mut x = v;
            loop {
                x = *(self.edge_to.get(&x).unwrap());
                if x == self.source { break };
                path.push(x);    
            }
            path.push(self.source);
            Some(path)
        }
    }
}

pub struct BreadthFirstSearch {
    source: usize,
    marked: Vec<usize>,
    edge_to: HashMap<usize, usize>
}

impl BreadthFirstSearch {
    pub fn new_search<T: GraphSearch>(g: &T, source: usize) -> BreadthFirstSearch {
        let mut bfs = BreadthFirstSearch{
            source: source,
            marked: Vec::new(),
            edge_to: HashMap::new()
        };
        bfs.bfs(g, source);

        bfs
    }

    fn bfs<T: GraphSearch>(&mut self, g: &T, source: usize) {
        let mut queue: VecDeque<usize> = VecDeque::new();
        // self.marked.push(source);
        queue.push_back(source);

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            for w in g.adj(v).unwrap().iter() {
                if !(self.marked.contains(w)) {
                    self.edge_to.insert(*w, v);
                    self.marked.push(*w);
                    queue.push_back(*w);
                }
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked.contains(&v)
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if self.has_path_to(v) == false {
            None
        } else {
            let mut path: Vec<usize> = Vec::new();
            path.push(v);

            let mut x = v;
            loop {
                x = *(self.edge_to.get(&x).unwrap());
                if x == self.source { break };
                path.push(x);    
            }
            path.push(self.source);
            Some(path)
        }
    }

    pub fn all_marked(&self) -> Option<Vec<usize>> {
        if self.marked.len() == 0 {
            None
        } else {
            Some(self.marked.clone())
        }
    }

    pub fn shortest_path<T: GraphSearch>(g: &T, source: usize, target: usize) -> Option<Vec<usize>> {
        let mut bfs = BreadthFirstSearch{
            source: source,
            marked: Vec::new(),
            edge_to: HashMap::new()
        };
        let mut queue: VecDeque<usize> = VecDeque::new();
        bfs.marked.push(source);
        queue.push_back(source);

        'outer: while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            for w in g.adj(v).unwrap().iter() {
                if *w == target {
                    bfs.edge_to.insert(*w, v);
                    bfs.marked.push(*w);
                    break 'outer;                    
                } else if !(bfs.marked.contains(w)) {
                    bfs.edge_to.insert(*w, v);
                    bfs.marked.push(*w);
                    queue.push_back(*w);
                }
            }
        }
        bfs.path_to(target)
    }
}
