use crate::header::Header;
use crate::oop_common::oop_constants;
use crate::oop_common::{OopCommonState, OopNavigation};

#[derive(Debug)]
pub struct OopSlice<'a> {
    index: usize,
    header: Header,
    extra_header: usize,
    contents: &'a mut [usize],
}

impl OopCommonState for OopSlice<'_> {
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

impl OopNavigation for OopSlice<'_> {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl<'a> OopSlice<'a> {
    // Constructor
    pub fn new(index: usize, contents: &'a mut [usize]) -> Self {
        let header = Header {
            header_value: contents[oop_constants::HEADER_INDEX],
        };
        let extra_header = if header.has_extra_slot_header() {
            contents[oop_constants::EXTRA_HEADER_INDEX]
        } else {
            oop_constants::NO_EXTRA_HEADER_VALUE
        };
        Self {
            index,
            header,
            extra_header,
            contents,
        }
    }

    pub fn become_free_oop(&mut self) {
        self.get_header_mut().become_free_oop();
        self.apply_header();
    }

    pub fn apply_header(&mut self) {
        self.contents[oop_constants::HEADER_INDEX] = self.header.header_value;
        if self.extra_header != oop_constants::NO_EXTRA_HEADER_VALUE {
            self.contents[oop_constants::EXTRA_HEADER_INDEX] = self.header_value()
        }
    }

    // We define the slots as 1 base.
    // This simplifies the small object case
    fn slot_bound_check(&self, an_index: usize) {
        if an_index < 1 || an_index > self.number_of_slots() {
            panic!("slot access was out of bound")
        }
    }

    fn compute_slot_index(&self, an_index: usize) -> usize {
        return if self.header.has_extra_slot_header() {
            oop_constants::EXTRA_HEADER_INDEX + an_index
        } else {
            an_index
        };
    }

    pub fn slot_at_index(&self, an_index: usize) -> usize {
        self.slot_bound_check(an_index);
        self.contents[self.compute_slot_index(an_index)]
    }

    pub fn slot_at_index_put(&mut self, an_index: usize, an_oop_address: usize) {
        self.slot_bound_check(an_index);
        self.contents[self.compute_slot_index(an_index)] = an_oop_address;
    }
}

#[cfg(test)]
mod tests {
    use crate::memory_space::MemorySpace;
    use crate::oop_builder::OopBuilder;
    use crate::oop_common::OopCommonState;
    use crate::oop_slice::OopSlice;

    #[test]
    fn become_free_oop_is_free_oop() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let oop_index = builder.build(&mut space);
        let mut new_object = space.get_oop_at(oop_index);

        new_object.become_free_oop();
        assert!(new_object.is_free_oop());
    }

    #[test]
    fn test_slot_at_index_returns_value() {
        let mut space = MemorySpace::for_bit_size(240);
        let mut builder = OopBuilder::new();
        builder.set_number_of_slots(1);
        let oop_index: usize = builder.build(&mut space);
        let slot_index: usize = 1;
        let slot_value: usize = 3;
        space[oop_index + slot_index] = slot_value;

		let oop: OopSlice = space.first_oop();
        assert_eq!(oop.slot_at_index(slot_index), slot_value);
    }
	
	#[test]
    fn test_slot_at_index_put_sets_value() {
        let mut space = MemorySpace::for_bit_size(240);
        let mut builder = OopBuilder::new();
        builder.set_number_of_slots(1);
		let mut oop: OopSlice = space.first_oop();
        let slot_index: usize = 1;
        let slot_value: usize = 3;
        oop.slot_at_index_put(slot_index, slot_value);

        assert_eq!(oop.slot_at_index(slot_index), slot_value);
    }

    #[test]
    fn test_big_oop_slot_at_index_returns_value() {
        let mut space = MemorySpace::for_bit_size(1000);
        let mut builder = OopBuilder::new();
        builder.set_number_of_slots(500);
		let mut oop: OopSlice = space.first_oop();
        let slot_index: usize = 250;
        let slot_value: usize = 42;
        oop.slot_at_index_put(slot_index, slot_value);

		assert!(oop.get_header().has_extra_slot_header());
        assert_eq!(oop.slot_at_index(slot_index), slot_value);
    }
}
