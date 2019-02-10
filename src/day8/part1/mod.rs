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
        self.metadata.iter().sum::<usize>()
                + self.children.iter()
                .map(|c| c.get_sum())
                .sum::<usize>()
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
