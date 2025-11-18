use std::{cell::RefCell, error::Error, rc::Rc};

struct StringBuf {
    // Multiple StringBufOperation implementations will need a mutable reference
    // concurrently to `s`, so we'll need to hand out Rc<RefCell<String>>s.
    s: Rc<RefCell<String>>,
}

impl StringBuf {
    fn new(s: String) -> Self {
        Self {
            s: Rc::new(RefCell::new(s)),
        }
    }

    fn get_mut(&self) -> Rc<RefCell<String>> {
        // Cloning Rcs is cheap. Note: this would NOT work in concurrent/multithreaded code.
        // try_borrow_mut will error out if there's already a mutable borrow oustanding.
        self.s.clone()
    }

    fn current_state(&self) -> String {
        self.s.borrow().to_string()
    }
}

trait StringBufOperation {
    fn execute(&mut self) -> Result<(), Box<dyn Error>>;
}

struct AddCharOperation {
    buf: Rc<RefCell<String>>,
    c: char,
}

impl AddCharOperation {
    fn new(buf: Rc<RefCell<String>>, c: char) -> Self {
        AddCharOperation { buf, c }
    }
}

impl StringBufOperation for AddCharOperation {
    fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf_ref = self.buf.try_borrow_mut()?;
        buf_ref.push_str(self.c.to_string().as_str());
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use crate::command::command::{AddCharOperation, StringBuf, StringBufOperation};

    #[test]
    fn test_add_char() {
        let sb = StringBuf::new("Hello Worl".to_string());
        let commands = [
            AddCharOperation::new(sb.get_mut(), 'd'),
            AddCharOperation::new(sb.get_mut(), '!'),
        ];

        for mut command in commands {
            command.execute().unwrap();
        }

        assert_eq!(sb.current_state(), "Hello World!".to_string())
    }
}
