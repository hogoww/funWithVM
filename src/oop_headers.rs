use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop_common::oop_constants;
use crate::oop_common::oop_utilities;
use crate::oop_common::{OopCommonState, OopNavigation};

#[derive(Debug)]
pub struct OopHeaders {
    index: usize,
    header: Header,
    extra_header: usize,
}

impl OopCommonState for OopHeaders {
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

impl OopNavigation for OopHeaders {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl OopHeaders {
    pub fn new(index: usize, space: &MemorySpace) -> Self {
        let header = Header {
            header_value: space[index + oop_constants::HEADER_INDEX],
        };
        let extra_header = if header.has_extra_slot_header() {
            space[index + oop_constants::EXTRA_HEADER_INDEX]
        } else {
            oop_constants::NO_EXTRA_HEADER_VALUE
        };
        Self {
            index,
            header,
            extra_header,
        }
    }

    //TODO(big oop)
    pub fn merge_with(&mut self, oop: OopHeaders, space: &mut MemorySpace) {
        // Merged oops only need one header !
        let total_size = self.oop_size() + oop.oop_size();
        let header_nb = oop_utilities::how_many_headers_for(total_size);
        let new_nb_slots = total_size - header_nb;
        self.set_number_of_slots(new_nb_slots);

        self.apply_header(space);
    }

    fn apply_header(&self, space: &mut MemorySpace) {
        space[self.get_index() + oop_constants::HEADER_INDEX] = self.header.header_value;
        if self.get_header().has_extra_slot_header() {
            space[self.get_index() + oop_constants::EXTRA_HEADER_INDEX] = self.get_extra_header()
        }
    }
}
