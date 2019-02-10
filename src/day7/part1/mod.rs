use regex::Regex;

type NodeIndex = usize;

#[derive(Debug)]
struct Node {
    name: char,
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
    ordered_steps_done: Vec<NodeIndex>,
}

impl Graph {
    fn add_node(&mut self, name: &char) -> NodeIndex {
        self.nodes.push(Node {
            name: name.clone(),
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

    fn get_node_index_or_create(&mut self, name: &char) -> NodeIndex {
        for (index, node) in self.nodes.iter().enumerate() {
            if &node.name == name {
                return index;
            }
        }

        self.add_node(&name)
    }

    fn finished(&self) -> bool {
        !self.nodes.iter().any(|n| !n.done)
    }

    fn work(&mut self) {
        let index= self.get_available_steps().iter()
                .map(|&i| (i, &self.nodes[i]))
                .min_by(|&a, &b| a.1.name.cmp(&b.1.name))
                .map(|t| t.0)
                .unwrap();

        self.nodes[index].done = true;
        self.ordered_steps_done.push(index);
    }

    fn get_available_steps(&self) -> Vec<NodeIndex> {
        let mut available_steps = Vec::new();

        for (index, _) in self.nodes.iter()
                .enumerate()
                .filter(|t| !t.1.done) {
            if !self.incoming_nodes(index)
                    .map(|i| self.nodes[i].done)
                    .any(|done| !done) {
                available_steps.push(index);
            }
        }

        available_steps
    }

    fn get_ordered_steps_done_as_string(&self) -> String {
        self.ordered_steps_done.iter()
                .map(|&i| self.nodes[i].name)
                .collect::<Vec<char>>()
                .iter()
                .collect()
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

pub fn solve(input: &str) -> String {
    let re = Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
    let mut graph = Graph { nodes: Vec::new(), edges: Vec::new(), ordered_steps_done: Vec::new() };

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let requisite_name = captures.get(1).unwrap().as_str().to_owned().chars().next().unwrap();
        let step_name = captures.get(2).unwrap().as_str().to_owned().chars().next().unwrap();

        let requisite_index = graph.get_node_index_or_create(&requisite_name);
        let node_index = graph.get_node_index_or_create(&step_name);
        graph.add_edge(requisite_index, node_index);
    }

    while !graph.finished() {
        graph.work();
    }

    graph.get_ordered_steps_done_as_string()
}

#[cfg(test)]
mod test;
