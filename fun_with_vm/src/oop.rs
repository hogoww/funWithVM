use crate::header::Header;
use crate::memory_space::MemorySpace;

#[derive(Debug)]
pub struct OopWithContents<'a> {
    index: usize,
    header: Header,
    extra_header: usize,
    contents: &'a mut [usize],
}

pub struct OopHeaders {
    index: usize,
    header: Header,
    extra_header: usize,
}

pub trait OopCommonState {
    fn get_index(&self) -> usize;
    fn get_header(&mut self) -> &mut Header;
    fn get_header_const(&self) -> &Header;
    fn get_extra_header(&self) -> usize;

    fn is_free_oop(&self) -> bool {
        self.get_header_const().is_free_oop()
    }

    fn header_value(&self) -> usize {
        self.get_header_const().header_value
    }

    //TODO(oop_size) try to extract this in its own trait
    //Unfortunately, repeated code with memory_space
    fn oop_size(&self) -> usize {
        self.get_header_const().header_size() + self.number_of_slots()
    }

    // Slots manipulation
    fn number_of_slots(&self) -> usize {
        if self.get_extra_header() != 0 {
            self.get_extra_header()
        } else {
            self.get_header_const().number_of_slots_bits()
        }
    }

    // Moving in space
    fn next_oop_index(&self) -> usize {
        self.get_index() + self.oop_size()
    }

    fn next_oop<'b>(&self, space: &'b mut MemorySpace) -> OopWithContents<'b> {
        space.get_oop_at(self.next_oop_index())
    }
}

impl OopCommonState for OopHeaders {
    fn get_index(&self) -> usize {
        self.index
    }
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }
    fn get_header_const(&self) -> &Header {
        &self.header
    }
    fn get_extra_header(&self) -> usize {
        self.extra_header
    }
}

impl OopHeaders {
    // Constructor
    pub fn new(index: usize, space: &MemorySpace) -> Self {
        let header = Header {
            header_value: space[index],
        };
        let extra_header = if header.has_extra_slot_header() {
            space[index + 1]
        } else {
            0
        };
        Self {
            index,
            header,
            extra_header,
        }
    }
}

impl OopCommonState for OopWithContents<'_> {
    fn get_index(&self) -> usize {
        self.index
    }
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }
    fn get_header_const(&self) -> &Header {
        &self.header
    }
    fn get_extra_header(&self) -> usize {
        self.extra_header
    }
}

impl<'a> OopWithContents<'a> {
    // Constructor
    pub fn new(index: usize, contents: &'a mut [usize]) -> Self {
        let header = Header {
            header_value: contents[0],
        };
        let extra_header = if header.has_extra_slot_header() {
            contents[1]
        } else {
            0
        };
        Self {
            index,
            header,
            extra_header,
            contents,
        }
    }

    // Constants
    //const HEADER_INDEX: usize = 0;
    pub const EXTRA_HEADER_INDEX: usize = 1;

    pub fn become_free_oop(&mut self) {
        self.get_header().become_free_oop();
        self.apply_header();
    }

    pub fn apply_header(&mut self) {
        self.contents[0] = self.header.header_value;
        if self.extra_header != 0 {
            self.contents[1] = self.header_value()
        }
    }

    fn slot_bound_check(&self, an_index: usize) {
        if an_index < 1 || an_index > self.number_of_slots() {
            panic!("slot access was out of bound")
        }
    }

    //TODO(bigoop)
    pub fn slot_at_index(&self, an_index: usize) -> usize {
        self.slot_bound_check(an_index);
        self.contents[an_index]
    }

    //TODO(bigoop)
    pub fn slot_at_put(&mut self, an_index: usize, an_oop_address: usize) {
        self.slot_bound_check(an_index);
        let slot_index = an_index;
        self.contents[slot_index] = an_oop_address;
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
