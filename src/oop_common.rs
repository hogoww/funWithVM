use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop_slice::OopSlice;

pub mod oop_constants {
    pub const HEADER_INDEX: usize = 0;
    pub const EXTRA_HEADER_INDEX: usize = 1;
    pub const NO_EXTRA_HEADER_VALUE: usize = 0;
}

pub trait OopCommonState {
    fn get_header(&self) -> &Header;
    fn get_header_mut(&mut self) -> &mut Header;
    fn get_extra_header(&self) -> usize;

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
