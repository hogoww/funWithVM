use crate::memory_space::MemorySpace;
use crate::memory_space_access::memory_space_access::*;
use crate::oop::*;
use crate::oop_common::OopCommonState;

pub struct MemorySpaceIterator {
    current_index: usize,
}

impl MemorySpaceIterator {
    pub fn new() -> Self {
        Self { current_index: 0 }
    }

    pub fn next<'a>(&mut self, space: &'a mut MemorySpace) -> Option<OopWithContents<'a>> {
        if self.current_index > space.get_end_index() {
            return None;
        }

        let res = oop_at_index(self.current_index, space);
        self.current_index = res.next_oop_index();
        Some(res)
    }
}

impl Default for MemorySpaceIterator {
    fn default() -> Self {
        Self::new()
    }
}

pub mod memory_space_access {
    use super::*;

    pub fn oop_at_index(index: usize, space: &mut MemorySpace) -> OopWithContents {
        let oop_size = space.oop_size_at(index);
        OopWithContents::new(index, &mut space[index..index + oop_size])
    }

    pub fn first_oop(space: &mut MemorySpace) -> OopWithContents {
        oop_at_index(0, space)
    }
}
