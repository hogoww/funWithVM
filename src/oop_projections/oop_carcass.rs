use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop_projections::oop_common::{oop_constants, OopCommonState};

#[derive(Debug, Default)]
pub struct OopCarcass {
    header: Header,
    extra_header: usize,
}

impl OopCommonState for OopCarcass {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
    fn get_extra_header(&self) -> usize {
        self.extra_header
    }
    fn set_extra_header(&mut self, new_value: usize) {
        self.extra_header = new_value;
    }
}

impl OopCarcass {
    pub fn new_from<T: OopCommonState>(oop: &T) -> Self {
        Self {
            header: Header {
                header_value: oop.header_value(),
            },
            extra_header: oop.get_extra_header(),
        }
    }

    pub fn apply_at_index_on_space(&self, index: usize, space: &mut MemorySpace) {
        space[index + oop_constants::HEADER_INDEX] = self.header.header_value;
        if self.header.has_extra_slot_header() {
            space[index + oop_constants::EXTRA_HEADER_INDEX] = self.number_of_slots();
        }
    }
}
