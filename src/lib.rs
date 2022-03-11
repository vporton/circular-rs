//! UNTESTED code.
//!
//! TODO: docs

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PositionID(u64);

impl PositionID {
    pub const ZERO: PositionID = PositionID(0);
}

pub struct Circular<T> {
    vec: Vec<T>,
    positions: HashMap<PositionID, Option<usize>>,
    next_pos_id: PositionID,
}

/// We have a vector and "position" therein.
///
/// TODO: More tests.
impl<T> Circular<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            positions: HashMap::new(),
            next_pos_id: PositionID(0),
        }
    }
    fn my_assert(&self) {
        for p in self.positions.values() {
            debug_assert!(p.is_none() || p.unwrap() < self.len())
        }
    }
    pub fn vec(&self) -> &Vec<T> {
        &self.vec
    }
    pub fn push(&mut self, value: T) {
        self.vec.push(value)
    }
    pub fn append(&mut self, other: &mut Vec<T>) {
        self.vec.append(other);
        self.my_assert();
    }
    pub fn remove_unsafe(&mut self, index: usize) -> T {
        assert!(index < self.len());
        self.my_assert();
        let result = self.vec.remove(index);
        let new_len = self.len();
        let empty = self.is_empty();
        for position in self.positions.values_mut() {
            if empty {
                *position = None;
            } else if let Some(ref mut pos) = *position {
                if *pos > index {
                    *pos -= 1;
                } else if *pos == new_len {
                    *pos = 0;
                }
            }
        }
        self.my_assert();
        result
    }
    pub fn remove_by_pos_id(&mut self, pos_id: PositionID) -> Option<T> {
        if let Some(pos) = self.positions[&pos_id] {
            self.my_assert();
            Some(self.remove_unsafe(pos))
        } else {
            self.my_assert();
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
    pub fn create_position(&mut self) -> PositionID {
        let result = self.next_pos_id;
        self.positions.insert(self.next_pos_id, None);
        self.next_pos_id.0 += 1;
        self.my_assert();
        result
    }
    pub fn destroy_position(&mut self, pos_id: PositionID) {
        self.positions.remove(&pos_id);
        self.my_assert();
    }
    pub fn get_position(&self, pos_id: PositionID) -> &Option<usize> {
        &self.positions[&pos_id]
    }
    pub fn set_position_unsafe(&mut self, pos_id: PositionID, index: Option<usize>) {
        self.positions.insert(pos_id, index);
        self.my_assert();
    }
    pub fn get_by_pos_id(&self, pos_id: PositionID) -> Option<&T> {
        self.positions[&pos_id].map(|pos| &self.vec[pos])
    }
    pub fn get_by_pos_id_mut(&mut self, pos_id: PositionID) -> Option<&mut T> {
        self.positions[&pos_id].map(|pos| &mut self.vec[pos])
    }
    /// If current is `None` tries to set it to `Some`.
    pub fn force_get_by_pos_id_mut(&mut self, pos_id: PositionID) -> Option<&mut T> {
        if let Some(pos) = self.positions[&pos_id] {
            self.my_assert();
            Some(&mut self.vec[pos])
        } else {
            self.init_position(pos_id)
        }
    }
    pub fn clear(&mut self) {
        self.vec.clear();
        for p in self.positions.values_mut() {
            *p = None;
        }
        self.my_assert();
    }

    pub async fn next(&mut self, pos_id: PositionID) -> Option<&T> {
        let pos = self.positions[&pos_id];
        if let Some(pos) = pos {
            self.positions.insert(pos_id, Some(if pos == self.vec.len() {
                0
            } else {
                pos + 1
            }));
            debug_assert!(pos < self.vec.len());
            Some(&self.vec[pos.clone()])
        } else {
            self.init_position(pos_id).map(|r| &*r)
        }
    }
    fn init_position(&mut self, pos_id: PositionID) -> Option<&mut T> {
        if self.vec.is_empty() {
            self.positions.insert(pos_id, None);
            self.my_assert();
            None
        } else {
            self.positions.insert(pos_id, Some(0));
            self.my_assert();
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
