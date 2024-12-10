use crate::memory_space::MemorySpace;
use crate::memory_space_access::memory_space_access::*;
use crate::oop_common::{OopCommonState, OopNavigation};
use crate::oop_headers::OopHeaders;
use crate::oop_slice::OopSlice;

pub struct MemorySpaceIterator {
    current_index: usize,
}

impl MemorySpaceIterator {
    pub fn new() -> Self {
        Self { current_index: 0 }
    }

    pub fn next<'a>(&mut self, space: &'a mut MemorySpace) -> Option<OopSlice<'a>> {
        if self.current_index > space.get_end_index() {
            return None;
        }

        let res = oop_at_index(self.current_index, space);
        self.current_index = res.next_oop_index();
        Some(res)
    }

    pub fn go_to_next(&mut self, space: &mut MemorySpace) {
        let res = oop_header_at_index(self.current_index, space);
        self.current_index = res.next_oop_index();
    }

    pub fn peak_next_headers(&self, space: &mut MemorySpace) -> Option<OopHeaders> {
        if self.current_index > space.get_end_index() {
            return None;
        }

        Some(oop_header_at_index(self.current_index, space))
    }

    pub fn next_headers(&mut self, space: &mut MemorySpace) -> Option<OopHeaders> {
        if self.current_index > space.get_end_index() {
            return None;
        }

        let res = oop_header_at_index(self.current_index, space);
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

    pub fn oop_header_at_index(index: usize, space: &mut MemorySpace) -> OopHeaders {
        OopHeaders::new(index, space)
    }

    pub fn first_oop_header(space: &mut MemorySpace) -> OopHeaders {
        oop_header_at_index(0, space)
    }

    pub fn oop_at_index(index: usize, space: &mut MemorySpace) -> OopSlice {
        let oop_size = OopHeaders::new(index, space).oop_size();
        OopSlice::new(index, &mut space[index..index + oop_size])
    }

    pub fn first_oop(space: &mut MemorySpace) -> OopSlice {
        oop_at_index(0, space)
    }
}
