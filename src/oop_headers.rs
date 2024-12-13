use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop_carcass::OopCarcass;
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

    pub fn become_free_oop(&mut self, space: &mut MemorySpace) {
        self.get_header_mut().become_free_oop();
        self.apply_header(space);
    }

    pub fn merge_with(&mut self, oop: OopHeaders, space: &mut MemorySpace) {
        // Expects both oops to be free, merging doesn't make sense otherwise
        let total_size = self.oop_size() + oop.oop_size();
        let header_nb = oop_utilities::how_many_headers_for(total_size);
        let new_nb_slots = total_size - header_nb;
        self.set_number_of_slots(new_nb_slots);

        self.apply_header(space);
    }

    pub fn carve_out(&self, size: usize) -> OopCarcass {
        // if the header grows, set_number_of_slots will add the extra header
        // If it needs to shrink, it's implicitly done
        let mut new_free_oop = OopCarcass::new_from(self);
        let new_resulting_size = self.oop_size() - size;
        let new_header_size = oop_utilities::how_many_headers_for(new_resulting_size);
        let new_slot_numbers = new_resulting_size - new_header_size;
        new_free_oop.set_number_of_slots(new_slot_numbers);

        new_free_oop
    }

    pub fn apply_header(&self, space: &mut MemorySpace) {
        space[self.get_index() + oop_constants::HEADER_INDEX] = self.header.header_value;
        if self.get_header().has_extra_slot_header() {
            space[self.get_index() + oop_constants::EXTRA_HEADER_INDEX] = self.get_extra_header()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oop_builder::OopBuilder;

    #[parameterized(nb_slots={ 2, 254, 255, 256, 257 })]
    fn test_merge_with(nb_slots: usize) {
        let mut space = MemorySpace::for_bit_size(1000);
        let mut builder = OopBuilder::new();
        builder.set_number_of_slots(nb_slots);
        let mut oop1 = OopHeaders::new(builder.build(&mut space), &mut space);
        oop1.become_free_oop(&mut space);
        let mut oop2 = OopHeaders::new(builder.build(&mut space), &mut space);
        oop2.become_free_oop(&mut space);

        let resulting_size = oop1.oop_size() + oop2.oop_size();

        oop1.merge_with(oop2, &mut space);

        assert!(oop1.is_free_oop());
        assert_eq!(oop1.oop_size(), resulting_size);
    }

    #[parameterized(memory_size={ 25, 254, 255, 256, 257, 300 })]
    fn test_carve_out(memory_size: usize) {
        let mut space = MemorySpace::for_bit_size(1000);
        let mut builder = OopBuilder::new();
        builder.set_number_of_slots(memory_size);
        let mut oop = OopHeaders::new(builder.build(&mut space), &mut space);
        oop.become_free_oop(&mut space);

        let carved_size: usize = 20;
        let carved_oop = oop.carve_out(carved_size);

        assert_eq!(carved_oop.oop_size() + carved_size, oop.oop_size());
    }
}
