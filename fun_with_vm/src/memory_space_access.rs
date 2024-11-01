use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop::Oop;
use crate::memory_space_access::memory_space_access::*;
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
        let res = oop_at_index(self.current_index, space);
        self.current_index = res.next_oop_index();
        Some(res)
    }
}

pub mod memory_space_access {
	use super::*;
	pub fn oop_at_index<'a>(index: usize, space: &'a mut MemorySpace) -> Oop<'a> {
		let header = Header {
			header_value: space[index],
		};
		let oop_size = header.oop_size();

		Oop::new(
			index,
			&mut space[index..index + oop_size],
		)
	}

	pub fn first_oop<'a>(space: &'a mut MemorySpace) -> Oop<'a> {
		oop_at_index(0, space)
	}
}
