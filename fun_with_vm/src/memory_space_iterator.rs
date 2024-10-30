use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop::Oop;

pub struct MemorySpaceIterator {
    current_index: usize,
}

impl MemorySpaceIterator {
    pub fn new() -> Self {
        Self { current_index: 0 }
    }

    pub fn next<'a>(&mut self, space: &'a mut MemorySpace) -> Option<Oop<'a>> {
        if self.current_index > space.get_end_index() {
            return None;
        }

        let header = Header {
            header_value: space[self.current_index],
        };
        let oop_size = header.oop_size();

        let oop_content = space.memory_slice(self.current_index, self.current_index + oop_size);

        let res: Oop = Oop::new(self.current_index, oop_content);
        self.current_index = res.next_oop_index();
        Some(res)
    }
}
