use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop_common::OopCommonState;

// Constants
const HEADER_INDEX: usize = 0;
const EXTRA_HEADER_INDEX: usize = 1;
const NO_EXTRA_HEADER_VALUE: usize = 0;

#[derive(Debug)]
pub struct OopWithContents<'a> {
    index: usize,
    header: Header,
    extra_header: usize,
    contents: &'a mut [usize],
}

impl OopCommonState for OopWithContents<'_> {
    fn get_index(&self) -> usize {
        self.index
    }
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

#[derive(Debug)]
pub struct OopHeaders {
    index: usize,
    header: Header,
    extra_header: usize,
}

impl OopCommonState for OopHeaders {
    fn get_index(&self) -> usize {
        self.index
    }
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

impl OopHeaders {
    // Constructor
    pub fn new(index: usize, space: &MemorySpace) -> Self {
        let header = Header {
            header_value: space[index + HEADER_INDEX],
        };
        let extra_header = if header.has_extra_slot_header() {
            space[index + EXTRA_HEADER_INDEX]
        } else {
            NO_EXTRA_HEADER_VALUE
        };
        Self {
            index,
            header,
            extra_header,
        }
    }
}

impl<'a> OopWithContents<'a> {
    // Constructor
    pub fn new(index: usize, contents: &'a mut [usize]) -> Self {
        let header = Header {
            header_value: contents[HEADER_INDEX],
        };
        let extra_header = if header.has_extra_slot_header() {
            contents[EXTRA_HEADER_INDEX]
        } else {
            NO_EXTRA_HEADER_VALUE
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
        self.contents[HEADER_INDEX] = self.header.header_value;
        if self.extra_header != NO_EXTRA_HEADER_VALUE {
            self.contents[EXTRA_HEADER_INDEX] = self.header_value()
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
            EXTRA_HEADER_INDEX + an_index
        } else {
            an_index
        };
    }

    pub fn slot_at_index(&self, an_index: usize) -> usize {
        self.slot_bound_check(an_index);
        self.contents[self.compute_slot_index(an_index)]
    }

    pub fn slot_at_put(&mut self, an_index: usize, an_oop_address: usize) {
        self.slot_bound_check(an_index);
        self.contents[self.compute_slot_index(an_index)] = an_oop_address;
    }
}

#[cfg(test)]
mod tests {
    use crate::memory_space::MemorySpace;
    use crate::oop::*;
    use crate::oop_builder::OopBuilder;

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

        let oop: OopWithContents = space.first_oop();
        assert_eq!(oop.slot_at_index(1), slot_value);
    }

    #[test]
    fn test_slot_at_put_sets_value() {
        let mut space = MemorySpace::for_bit_size(240);
        let mut builder = OopBuilder::new();
        builder.set_number_of_slots(1);
        builder.build(&mut space);
        let slot_index: usize = 1;
        let slot_value: usize = 3;

        let mut oop: OopWithContents = space.first_oop();
        oop.slot_at_put(slot_index, slot_value);
        assert_eq!(oop.slot_at_index(1), slot_value);
    }
}
