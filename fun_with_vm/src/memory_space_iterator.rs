use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop::Oop;
use core::iter::Iterator;

pub struct MemorySpaceIterator<'a> {
    space: &'a MemorySpace,
    current_index: usize,
}

impl<'a> MemorySpaceIterator<'a> {
    pub fn new(space: &'a mut MemorySpace) -> Self {
        Self {
            space,
            current_index: 1,
        }
    }
}

impl<'a> Iterator for MemorySpaceIterator<'a> {
    type Item = Oop;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index > self.space.get_end_index() {
            return None;
        }

        let header = Header {
            header_value: self.space[self.current_index],
        };
        let mut oop_content: Vec<usize> = vec![0; header.oop_size()];
        oop_content.copy_from_slice(
            self.space
                .memory_slice(self.current_index, self.current_index + header.oop_size()),
        );
        let res = Oop::new(self.current_index, oop_content);
        self.current_index = res.next_oop_index();
        Some(res)
    }
}
