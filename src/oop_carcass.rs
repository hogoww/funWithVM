use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop_common::{oop_constants, OopCommonState};

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
}

impl OopCarcass {
    pub fn new_from<T: OopCommonState>(oop: T) -> Self {
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

    pub fn set_number_of_slots(&mut self, number_of_slots: usize) {
        if self.number_of_slots() > Header::MAX_NUMBER_OF_SLOTS {
            self.header.set_number_of_slots_to_max();
            self.extra_header = number_of_slots;
        } else {
            self.header.set_number_of_slots_bits(number_of_slots);
        }
    }
}
