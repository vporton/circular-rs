//! UNTESTED code.
//!
//! TODO: docs

pub struct Circular<T: Clone> {
    vec: Vec<T>,
    position: Option<usize>,
}

/// We have a vector and "position" therein.
///
/// TODO: More tests.
impl<T: Clone> Circular<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            position: None,
        }
    }
    pub fn push(&mut self, value: T) {
        self.vec.push(value)
    }
    pub fn append(&mut self, other: &mut Vec<T>) {
        self.vec.append(other)
    }
    pub fn remove_current(&mut self) -> Option<T> {
        if let Some(ref mut pos) = self.position {
            let result = self.vec.remove(pos.clone());
            if *pos == self.vec.len() {
                *pos = 0;
            }
            Some(result)
        } else {
            self.position = Some(0);
            self.vec.get(0).map(|v| v.clone())
        }
    }
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let result = self.vec.remove(index);
        if let Some(ref mut pos) = self.position {
            if *pos > index {
                *pos -= 1;
            }
        }
        if self.is_empty() {
            self.position = None;
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.vec.iter_mut()
    }
    pub fn get_position(&self) -> Option<usize> {
        self.position
    }
    pub fn set_position(&mut self, pos: Option<usize>) {
        self.position = pos;
    }
    pub fn get_current(&self) -> Option<&T> {
        self.position.map(|pos| &self.vec[pos])
    }
    pub fn get_current_mut(&mut self) -> Option<&mut T> {
        self.position.map(|pos| &mut self.vec[pos])
    }
    /// If current is `None` tries to set it to `Some`.
    pub fn force_get_current(&mut self) -> Option<&T> {
        if let Some(pos) = self.position {
            Some(&self.vec[pos])
        } else {
            self.init_position().map(|r| &*r)
        }
    }
    /// If current is `None` tries to set it to `Some`.
    pub fn force_get_current_mut(&mut self) -> Option<&mut T> {
        if let Some(pos) = self.position {
            Some(&mut self.vec[pos])
        } else {
            self.init_position()
        }
    }
    pub fn clear(&mut self) {
        self.vec.clear();
        self.position = None;
    }

    pub async fn next(&mut self) -> Option<&T> {
        if let Some(ref mut pos) = self.position {
            *pos += 1;
            if *pos == self.vec.len() {
                *pos = 0;
            }
            Some(&self.vec[pos.clone()])
        } else {
            self.init_position().map(|r| &*r)
        }
    }
    fn init_position(&mut self) -> Option<&mut T> {
        if self.vec.is_empty() {
            self.position = None;
            None
        } else {
            self.position = Some(0);
            Some(&mut self.vec[0])
        }
    }
}

/// Tests do not pass.
#[cfg(test)]
mod tests {
    use crate::{Circular};

    #[test]
    fn one_position_middle() {
        let mut v = Circular::new();
        let mut input = (0..10).collect::<Vec<i32>>();
        v.append(&mut input);
        v.set_position(Some(5));
        v.remove_current();
        assert_eq!(v.iter().map(|n| *n).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4, 6, 7, 8, 9]);
        assert_eq!(v.get_position(), Some(5));
    }
}
