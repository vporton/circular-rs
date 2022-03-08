//! UNTESTED code.
//!
//! TODO: docs

pub struct Circular<T: Clone> {
    vec: Vec<T>,
    position: Option<usize>,
}

/// We have a vector of "resources", "allocated" positions therein, and "next" resource to be allocated.
/// There is the operation to replace a position in the vector of positions
/// by the available position.
///
/// Example: Several threads use a pool of network nodes to download from.
/// From the pool we "view" a range of currently used nodes, one by thread.
/// If a note is invalidated, it is removed from the list.
/// Nodes later than it in the range decrease their positions.
///
/// TODO: Test it.
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
    pub fn clear(&mut self) {
        self.vec.clear();
        self.position = None;
    }

    pub async fn next(&mut self) -> Option<T> {
        if let Some(ref mut pos) = self.position {
            *pos += 1;
            if *pos == self.vec.len() {
                *pos = 0;
            }
            Some(self.vec[pos.clone()].clone())
        } else {
            if self.vec.is_empty() {
                self.position = None;
                None
            } else {
                self.position = Some(0);
                Some(self.vec[0].clone())
            }
        }
    }
}

/// Tests do not pass.
#[cfg(test)]
mod tests {
    use crate::{Position, Circular, VecWithPositions};

    #[test]
    fn one_position_before() {
        let mut v = Circular::new();
        let mut input = (0..10).collect::<Vec<i32>>();
        v.append(&mut input);
        v.set_position(Some(Position(3)));
        v.remove(Position(5));
        assert_eq!(v.inactive_iter().map(|n| *n).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4, 6, 7, 8, 9]);
        assert_eq!(v.get_position(), Some(Position(3)));
    }

    #[test]
    fn one_position_middle() {
        let mut v = Circular::new();
        let mut input = (0..10).collect::<Vec<i32>>();
        v.append(&mut input);
        v.set_position(Some(Position(5)));
        v.remove(Position(5));
        assert_eq!(v.inactive_iter().map(|n| *n).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4, 6, 7, 8, 9]);
        assert_eq!(v.get_position(), Some(Position(5)));
    }

    #[test]
    fn one_position_after() {
        let mut v = Circular::new();
        let mut input = (0..10).collect::<Vec<i32>>();
        v.append(&mut input);
        v.set_position(Some(Position(7)));
        v.remove(Position(5));
        assert_eq!(v.inactive_iter().map(|n| *n).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4, 6, 7, 8, 9]);
        assert_eq!(v.get_position(), Some(Position(6)));
    }
}
