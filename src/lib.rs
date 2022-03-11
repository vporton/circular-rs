//! UNTESTED code.
//!
//! TODO: docs

pub struct Circular<T> {
    vec: Vec<T>,
    positions: Vec<Option<usize>>,
}

#[derive(Clone, Copy)]
pub struct PositionID(pub usize);

/// We have a vector and "position" therein.
///
/// TODO: More tests.
impl<T> Circular<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            positions: Vec::new(),
        }
    }
    pub fn vec(&self) -> &Vec<T> {
        &self.vec
    }
    pub fn push(&mut self, value: T) {
        self.vec.push(value)
    }
    pub fn append(&mut self, other: &mut Vec<T>) {
        self.vec.append(other)
    }
    pub fn remove_unsafe(&mut self, index: usize) -> T {
        let result = self.vec.remove(index);
        let empty = self.is_empty();
        for position in self.positions.iter_mut() {
            if empty {
                *position = None;
            } else if let Some(ref mut pos) = *position {
                if *pos > index {
                    *pos -= 1;
                }
            }
        }
        result
    }
    pub fn remove_by_pos_id(&mut self, pos_id: PositionID) -> Option<T> {
        if let Some(pos) = self.positions[pos_id.0] {
            Some(self.remove_unsafe(pos))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn positions_is_empty(&self) -> bool {
        self.positions.is_empty()
    }
    pub fn positions_len(&self) -> usize {
        self.positions.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.vec.iter_mut()
    }
    pub fn push_position(&mut self) -> PositionID {
        let result = PositionID(self.positions.len());
        self.positions.push(None);
        result
    }
    pub fn pop_position(&mut self) {
        self.positions.pop();
    }
    pub fn get_position(&self, pos_id: PositionID) -> &Option<usize> {
        &self.positions[pos_id.0]
    }
    pub fn get_position_mut(&mut self, pos_id: PositionID) -> &mut Option<usize> {
        &mut self.positions[pos_id.0]
    }
    pub fn set_position_unsafe(&mut self, pos_id: PositionID, index: Option<usize>) {
        self.positions[pos_id.0] = index;
    }
    pub fn get_by_pos_id(&self, pos_id: PositionID) -> Option<&T> {
        self.positions[pos_id.0].map(|pos| &self.vec[pos])
    }
    pub fn get_by_pos_id_mut(&mut self, pos_id: PositionID) -> Option<&mut T> {
        self.positions[pos_id.0].map(|pos| &mut self.vec[pos])
    }
    /// If current is `None` tries to set it to `Some`.
    pub fn force_get_by_pos_id_mut(&mut self, pos_id: PositionID) -> Option<&mut T> {
        if let Some(pos) = self.positions[pos_id.0] {
            Some(&mut self.vec[pos])
        } else {
            self.init_position(pos_id)
        }
    }
    pub fn clear(&mut self) {
        self.vec.clear();
        for p in self.positions.iter_mut() {
            *p = None;
        }
    }

    pub async fn next(&mut self, pos_id: PositionID) -> Option<&T> {
        if let Some(ref mut pos) = self.positions[pos_id.0] {
            *pos += 1;
            if *pos == self.vec.len() {
                *pos = 0;
            }
            Some(&self.vec[pos.clone()])
        } else {
            self.init_position(pos_id).map(|r| &*r)
        }
    }
    fn init_position(&mut self, pos_id: PositionID) -> Option<&mut T> {
        if self.vec.is_empty() {
            self.positions[pos_id.0] = None;
            None
        } else {
            self.positions[pos_id.0] = Some(0);
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
        v.set_position_unsafe(Some(5));
        v.remove_current();
        assert_eq!(v.iter().map(|n| *n).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4, 6, 7, 8, 9]);
        assert_eq!(v.get_position(), Some(5));
    }
}
