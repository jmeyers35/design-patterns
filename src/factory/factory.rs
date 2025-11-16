pub trait BinaryOperation {
    fn do_op(&self, x: f64, y: f64) -> f64;
}

pub trait BinaryOperationFactory {
    fn get_binary_op(&self) -> Box<dyn BinaryOperation>;
}

struct AdditionFactory {}

struct Addition {}

impl BinaryOperation for Addition {
    fn do_op(&self, x: f64, y: f64) -> f64 {
        x + y
    }
}

impl BinaryOperationFactory for AdditionFactory {
    fn get_binary_op(&self) -> Box<dyn BinaryOperation> {
        Box::new(Addition {})
    }
}

pub enum OperationKind {
    Add,
}

pub fn get_factory(kind: OperationKind) -> impl BinaryOperationFactory {
    match kind {
        OperationKind::Add => AdditionFactory {},
    }
}

#[cfg(test)]
pub mod tests {
    use super::{BinaryOperationFactory, OperationKind, get_factory};

    #[test]
    fn test_addition() {
        let factory = get_factory(OperationKind::Add);
        let op = factory.get_binary_op();
        assert_eq!(op.do_op(1.5, 1.5), 3.0)
    }
}
