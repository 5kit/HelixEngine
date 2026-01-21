use pyo3::prelude::*;

/*
 * GENERAL OBJECT STORAGE Class
 * ECS Genric Storage Type
 * Allows for single existance of objects and reuse of object slots via genrations
 * generations also invalidate outdated handlers.
 */

pub struct ObjectStorage<T> {
    slots: Vec<Option<T>>,
    slot_gen: Vec<u32>,
    free: Vec<usize>,
}

impl<T> ObjectStorage<T> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),    // Where Objects of type T are Stored
            slot_gen: Vec::new(), // any handler genration < current gen is outdated
            free: Vec::new(),     // free space Stack
        }
    }

    // insert value into next available slot
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

    pub fn resolve(&self, h: &Handle) -> Option<&T> {
        if *self.slot_gen.get(h.index)? == h.generation {
            self.slots[h.index].as_ref()
        } else {
            None
        }
    }

    pub fn resolve_mut(&mut self, h: &Handle) -> Option<&mut T> {
        if *self.slot_gen.get(h.index)? == h.generation {
            self.slots[h.index].as_mut()
        } else {
            None
        }
    }
}

/*
 * GENRAL OBJECT HANDLER Class
 * The Handler Class are genral handlers that work with the ObjectStorage class.
 * These are used in wrappers for object type specifications,
 * And also are used for python exposing.
 * e.g. PyMeshHandle
 *
 */

#[pyclass]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Handle {
    pub index: usize, // ID
    pub generation: u32,
}
