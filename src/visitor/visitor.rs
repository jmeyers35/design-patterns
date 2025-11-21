pub trait Visitor {
    fn visit_parent(&mut self, parent: &Parent);
    fn visit_leaf(&mut self, leaf: &Leaf);
}

pub trait Visitable {
    fn accept(&self, visitor: &mut dyn Visitor);
}

pub struct Leaf {
    data: usize,
}

pub struct Parent<'a> {
    child: Option<&'a Leaf>,
    data: usize,
}

impl<'a> Visitable for Parent<'a> {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_parent(self);
        if let Some(child) = self.child {
            visitor.visit_leaf(child);
        }
    }
}

impl Visitable for Leaf {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_leaf(self);
    }
}

pub struct SumVisitor {
    total: usize,
}

impl SumVisitor {
    pub fn new() -> Self {
        Self { total: 0 }
    }

    pub fn total(&self) -> usize {
        self.total
    }
}

impl Visitor for SumVisitor {
    fn visit_parent(&mut self, parent: &Parent) {
        self.total += parent.data
    }

    fn visit_leaf(&mut self, leaf: &Leaf) {
        self.total += leaf.data
    }
}

#[cfg(test)]
mod tests {
    use super::{Leaf, Parent, SumVisitor, Visitable};

    #[test]
    fn it_works() {
        let leaf = Leaf { data: 2 };
        let parent = Parent {
            child: Some(&leaf),
            data: 3,
        };

        let mut visitor = SumVisitor::new();
        parent.accept(&mut visitor);

        assert_eq!(visitor.total(), 5);
    }
}
