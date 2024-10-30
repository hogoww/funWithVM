use crate::header::Header;
use crate::memory_space::MemorySpace;

#[derive(Debug)]
pub struct Oop<'a> {
    index: usize,
    header: Header,
    contents: &'a mut [usize],
}

impl<'a> Oop<'a> {
    // Constructor
    pub fn new(index: usize, contents: &'a mut [usize]) -> Self {
        let header = Header {
            header_value: contents[0],
        };
        Self {
            index,
            contents,
            header,
        }
    }

    // Constant
    fn header_index(&self) -> usize {
        0
    }

    // Accessors
    pub fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    //shortcut
    pub fn header_value(&self) -> usize {
        self.contents[self.header_index()]
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    // Testing
    pub fn is_free_oop(&self) -> bool {
        self.header.is_free_oop()
    }

    pub fn become_free_oop(&mut self) {
        self.header.become_free_oop();
        self.apply_header();
    }

    // Apply on space
    pub fn apply_to_space(&mut self) {
        self.apply_header();
        //self.apply_slots();
    }

    pub fn apply_header(&mut self) {
        let header_index = self.header_index();
        self.contents[header_index] = self.header.header_value;
    }

    //pub fn apply_slots(&self) {
    //  let mut index = self.index;
    // for value in self.contents {
    //         self.contents[index] = *value;
    //         index += 1;
    //     }
    // }

    // Moving in space
    pub fn next_oop_index(&self) -> usize {
        self.index + self.header.oop_size()
    }

    pub fn next_oop<'b>(&self, space: &'b mut MemorySpace) -> Oop<'b> {
        space.get_oop_at(self.next_oop_index())
    }

    // Slots manipulation
    pub fn number_of_slots(&self) -> usize {
        self.header.number_of_slots_bits()
    }

    fn slot_bound_check(&self, an_index: usize) {
        if an_index < 1 || an_index > self.number_of_slots() {
            panic!("slot access was out of bound")
        }
    }

    pub fn slot_at_index(&self, an_index: usize) -> usize {
        self.slot_bound_check(an_index);
        self.contents[an_index]
    }

    pub fn slot_at_put(&mut self, an_index: usize, an_oop_address: usize) {
        self.slot_bound_check(an_index);
        let slot_index = self.header_index() + an_index;
        self.contents[slot_index] = an_oop_address;
    }
}

#[cfg(test)]
mod tests {
    use crate::memory_space::MemorySpace;
    use crate::oop::Oop;
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

        let oop: Oop = space.first_oop();
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

        let mut oop: Oop = space.first_oop();
        oop.slot_at_put(slot_index, slot_value);
        assert_eq!(oop.slot_at_index(1), slot_value);
    }
}
