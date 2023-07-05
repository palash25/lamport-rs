use std::{sync::Mutex};

#[derive(Debug)]
struct Node {
    name: String,
    timestamp: Mutex<i64>
}

#[allow(unused)]
impl Node {
    pub fn new(name: String) -> Self {
        Node { name: name, timestamp: Mutex::from(0) }
    }

    fn tick(&mut self) -> Result<(i64), ()> {
        let mut t = match self.timestamp.lock() {
            Err(_) => return Err(()),
            Ok(m) => m,
        };
        *t += 1;

        Ok((*t))
    }

    pub fn local_event(&mut self) {
        self.tick().unwrap();
    }

    pub fn current_timestamp(&self) -> Result<(i64), ()> {
        let mut t = match self.timestamp.lock() {
            Err(_) => return Err(()),
            Ok(m) => m,
        };

        Ok((t.clone()))
    }

    pub fn send_message(&mut self) {
        self.tick().unwrap();

        // send message
    }

    pub fn receive_message(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut node = Node::new(String::from("test"));
        let t = node.tick().unwrap();
        assert_eq!(t, 1);
        assert_eq!(node.current_timestamp().unwrap(), 1);
        node.local_event();
        assert_eq!(node.current_timestamp().unwrap(), 2);
    }
}
