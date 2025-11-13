// A Node exposes a calculation over its entire hierarchy.
pub trait Node {
    fn calculate(&self) -> u64;
    fn add(&mut self, child: Box<dyn Node>);
    fn get_child(&self, idx: usize) -> Option<&dyn Node>;
}

pub struct LeafNode {
    val: u64,
}

impl LeafNode {
    pub fn new(val: u64) -> Self {
        Self { val }
    }
}

pub struct ParentNode {
    val: u64,
    children: Vec<Box<dyn Node>>,
}

impl ParentNode {
    pub fn new(val: u64) -> Self {
        Self {
            val,
            children: Vec::new(),
        }
    }
}

impl Node for LeafNode {
    fn calculate(&self) -> u64 {
        self.val
    }

    fn add(&mut self, _child: Box<dyn Node>) {
        // no-op; leaves cannot have children
    }

    fn get_child(&self, _idx: usize) -> Option<&dyn Node> {
        None
    }
}

impl Node for ParentNode {
    fn calculate(&self) -> u64 {
        self.children
            .iter()
            .fold(self.val, |acc, child| acc + child.calculate())
    }

    fn add(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }

    fn get_child(&self, idx: usize) -> Option<&dyn Node> {
        self.children.get(idx).map(|child| child.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::{LeafNode, Node, ParentNode};

    #[test]
    fn calculates_nested_sum() {
        let mut root = ParentNode::new(10);

        root.add(Box::new(LeafNode::new(5)));
        root.add(Box::new(LeafNode::new(15)));

        let mut branch = ParentNode::new(20);
        branch.add(Box::new(LeafNode::new(3)));
        branch.add(Box::new(LeafNode::new(7)));

        root.add(Box::new(branch));

        assert_eq!(root.calculate(), 60);
    }
}
