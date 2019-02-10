#[derive(Default, Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn decode(iterator: &mut impl Iterator<Item = usize>) -> Node {
        let amount_children = iterator.next().unwrap();
        let amount_metadata = iterator.next().unwrap();
        let mut node = Node::default();

        for _ in 0..amount_children {
            node.children.push(Self::decode(iterator));
        }

        for _ in 0..amount_metadata {
            node.metadata.push(iterator.next().unwrap());
        }

        node
    }

    fn get_sum(&self) -> usize {
        if self.children.is_empty() {
            return self.metadata.iter().sum::<usize>()
        }

        let mut sum_children = 0;

        for &metadata in self.metadata.iter() {
            if metadata <= self.children.len() {
                sum_children += self.children.get(metadata - 1).unwrap().get_sum();
            }
        }

        sum_children
    }
}

pub fn solve(input: &str) -> usize {
    let entries: Vec<usize> = input.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

    Node::decode(&mut entries.into_iter()).get_sum()
}

#[cfg(test)]
mod test;
