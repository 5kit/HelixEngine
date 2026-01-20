use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Handle {
    pub index: usize,
    pub generation: u32,
}

pub struct ObjectStorage<T> {
    slots: Vec<Option<T>>,
    slot_gen: Vec<u32>,
    free: Vec<usize>,
}

impl<T> ObjectStorage<T> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            slot_gen: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> Handle {
        if let Some(index) = self.free.pop() {
            self.slots[index] = Some(value);
            Handle {
                index,
                generation: self.slot_gen[index],
            }
        } else {
            let index = self.slots.len();
            self.slots.push(Some(value));
            self.slot_gen.push(0);
            Handle {
                index,
                generation: 0,
            }
        }
    }

    pub fn remove(&mut self, h: Handle) -> Option<T> {
        if self.slot_gen.get(h.index)? != &h.generation {
            // Out of date handle
            return None;
        }

        let value = self.slots[h.index].take();
        self.slot_gen[h.index] += 1;
        self.free.push(h.index);
        value
    }
}
