use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop_slice::OopSlice;

pub mod oop_constants {
    pub const HEADER_INDEX: usize = 0;
    pub const EXTRA_HEADER_INDEX: usize = 1;
    pub const NO_EXTRA_HEADER_VALUE: usize = 0;
}

pub mod oop_utilities {
    use crate::header::Header;

    pub fn how_many_headers_for(some_memory_size: usize) -> usize {
        if some_memory_size < Header::MAX_NUMBER_OF_SLOTS {
            1
        } else {
            2
        }
    }
}

pub trait OopCommonState {
    fn get_header(&self) -> &Header;
    fn get_header_mut(&mut self) -> &mut Header;
    fn get_extra_header(&self) -> usize;
    fn set_extra_header(&mut self, index: usize);

    fn is_free_oop(&self) -> bool {
        self.get_header().is_free_oop()
    }

    fn header_value(&self) -> usize {
        self.get_header().header_value
    }

    fn oop_size(&self) -> usize {
        self.get_header().header_size() + self.number_of_slots()
    }

    // Slots manipulation
    fn number_of_slots(&self) -> usize {
        if self.get_header().has_extra_slot_header() {
            self.get_extra_header()
        } else {
            self.get_header().number_of_slots_bits()
        }
    }

    fn set_number_of_slots(&mut self, number_of_slots: usize) {
        if number_of_slots > Header::MAX_NUMBER_OF_SLOTS {
            self.get_header_mut().set_number_of_slots_to_max();
            self.set_extra_header(number_of_slots);
        } else {
            self.get_header_mut()
                .set_number_of_slots_bits(number_of_slots);
        }
    }
}

pub trait OopNavigation: OopCommonState {
    fn get_index(&self) -> usize;

    fn next_oop_index(&self) -> usize {
        self.get_index() + self.oop_size()
    }

    fn next_oop<'b>(&self, space: &'b mut MemorySpace) -> OopSlice<'b> {
        space.get_oop_at(self.next_oop_index())
    }
}
