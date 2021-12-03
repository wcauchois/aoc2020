use std::{cell::Cell, rc::Rc};

pub struct MessageReader {
    chars: Vec<char>,
    pos: Rc<Cell<usize>>,
}

pub struct SaveHandle {
    saved_pos: usize,
    parent_pos: Rc<Cell<usize>>,
}

impl SaveHandle {
    pub fn restore(&self) {
        self.parent_pos.set(self.saved_pos);
    }
}

impl MessageReader {
    pub fn new(s: &str) -> Self {
        MessageReader {
            chars: s.chars().collect(),
            pos: Rc::new(Cell::new(0)),
        }
    }

    pub fn save(&self) -> SaveHandle {
        SaveHandle {
            saved_pos: self.pos.get(),
            parent_pos: self.pos.clone(),
        }
    }

    pub fn peek(&self) -> Option<char> {
        let pos = self.pos.get();
        if pos < self.chars.len() {
            Some(self.chars[pos])
        } else {
            None
        }
    }

    pub fn next(&self) -> Option<char> {
        let pos = self.pos.get();
        if pos < self.chars.len() {
            self.pos.set(pos + 1);
            Some(self.chars[pos])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        // XXX doesn't pass anymore
        let reader = MessageReader::new("abc");
        assert_eq!(reader.next(), Some('a'));
        {
            let _handle = reader.save();
            assert_eq!(reader.next(), Some('b'));
        }
        // Should return 'b' again since the position was restored.
        assert_eq!(reader.next(), Some('b'));
    }
}
