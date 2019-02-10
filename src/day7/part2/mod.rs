use regex::Regex;

type NodeIndex = usize;

#[derive(Debug)]
struct Node {
    name: char,
    duration: usize,
    elapsed_seconds: usize,
    done: bool,
    first_incoming_edge: Option<EdgeIndex>,
}

type EdgeIndex = usize;

#[derive(Debug)]
struct Edge {
    target: NodeIndex,
    next_edge: Option<EdgeIndex>,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    workers: Vec<Worker>,
    elapsed_seconds: usize,
}

impl Graph {
    fn add_node(&mut self, name: &char, duration: usize) -> NodeIndex {
        self.nodes.push(Node {
            name: name.clone(),
            duration,
            elapsed_seconds: 0,
            done: false,
            first_incoming_edge: None,
        });

        self.nodes.len() - 1
    }

    fn add_edge(&mut self, source_index: NodeIndex, target_index: NodeIndex) {
        let target_node = &mut self.nodes[target_index];

        self.edges.push(Edge {
            target: source_index,
            next_edge: target_node.first_incoming_edge,
        });

        target_node.first_incoming_edge = Some(self.edges.len() - 1);
    }

    fn incoming_nodes(&self, source: NodeIndex) -> AdjacentNodes {
        AdjacentNodes {
            graph: self,
            current_edge_index: self.nodes[source].first_incoming_edge,
        }
    }

    fn get_node_index_or_create(&mut self, name: &char, duration: usize) -> NodeIndex {
        for (index, node) in self.nodes.iter().enumerate() {
            if &node.name == name {
                return index;
            }
        }

        self.add_node(&name, duration)
    }

    fn finished(&self) -> bool {
        !self.nodes.iter().any(|n| !n.done)
    }

    fn work(&mut self) {
        self.process_working_workers();
        self.process_idle_workers();
    }

    fn process_working_workers(&mut self) {
        for worker in self.workers.iter_mut().filter(|w| w.is_working) {
            let mut node = &mut self.nodes[worker.step_index.unwrap()];
            node.elapsed_seconds += 1;

            if node.elapsed_seconds == node.duration {
                node.done = true;
                worker.is_working = false;
                worker.step_index = None;
            }
        }
    }

    fn process_idle_workers(&mut self) {
        let apt_workers_indices = self.get_apt_workers_indices();

        for index in apt_workers_indices {
            self.process_worker(index);
        }
    }

    fn get_apt_workers_indices(&self) -> Vec<usize> {
        let mut apt_workers_indices = Vec::new();

        for (index, worker) in self.workers.iter().enumerate() {
            if worker.is_working {
                continue;
            }

            apt_workers_indices.push(index);
        }

        apt_workers_indices
    }

    fn process_worker(&mut self, worker_index: usize) {
        match self.get_next_available_step() {
            Some(index) => {
                self.workers[worker_index].is_working = true;
                self.workers[worker_index].step_index = Some(index);
            },

            None => {},
        }
    }

    fn get_next_available_step(&self) -> Option<NodeIndex> {
        self.get_available_steps().iter()
                .map(|&i| (i, &self.nodes[i]))
                .min_by(|&a, &b| a.1.name.cmp(&b.1.name))
                .map(|t| t.0)
    }

    fn get_available_steps(&self) -> Vec<NodeIndex> {
        let mut available_steps = Vec::new();

        for (index, _) in self.nodes.iter()
                .enumerate()
                .filter(|t| !t.1.done) {
            if !self.incoming_nodes(index)
                    .map(|i| self.nodes[i].done)
                    .any(|done| !done)
                    && !self.workers.iter()
                    .filter(|w| w.step_index.is_some())
                    .any(|w| w.step_index.unwrap() == index){
                available_steps.push(index);
            }
        }

        available_steps
    }
}

struct AdjacentNodes<'g> {
    graph: &'g Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'g> Iterator for AdjacentNodes<'g> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,

            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index];
                self.current_edge_index = edge.next_edge;
                Some(edge.target)
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    is_working: bool,
    step_index: Option<NodeIndex>,
}

pub fn solve(input: &str, workers_amount: usize, duration_offset: usize) -> usize {
    let re = Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
    let mut workers = Vec::new();

    for _ in 0..workers_amount {
        workers.push(Worker {
            is_working: false,
            step_index: None }
        );
    }

    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        workers,
        elapsed_seconds: 0
    };

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let requisite_name = captures.get(1).unwrap().as_str().to_owned().chars().next().unwrap();
        let step_name = captures.get(2).unwrap().as_str().to_owned().chars().next().unwrap();

        let requisite_duration = requisite_name as usize - duration_offset - 4;
        let step_duration = step_name as usize - duration_offset - 4;

        let requisite_index = graph.get_node_index_or_create(&requisite_name, requisite_duration);
        let node_index = graph.get_node_index_or_create(&step_name, step_duration);
        graph.add_edge(requisite_index, node_index);
    }

    loop {
        graph.work();

        if graph.finished() {
            break;
        }

        graph.elapsed_seconds += 1;
    }

    graph.elapsed_seconds
}

#[cfg(test)]
mod test;
