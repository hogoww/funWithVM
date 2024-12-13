use crate::allocator::where_to_allocate;
use crate::memory_space::MemorySpace;
use crate::oop_carcass::OopCarcass;
use crate::oop_common::OopCommonState;
use crate::oop_headers::OopHeaders;

pub struct OopBuilder {
    number_of_slots: usize,
    class_index: usize,
}

impl OopBuilder {
    pub fn new() -> OopBuilder {
        Self {
            class_index: 2,
            number_of_slots: 0,
        }
    }

    pub fn initialize(&mut self) {
        //The class index should probably be different than existing classes (i.e., freeObject)
        //Maybe it should always be required
        self.class_index = 2;
        self.number_of_slots = 0;
    }

    // API, for code readability
    pub fn reset(&mut self) {
        self.initialize();
    }

    // This allows to bypass the allocation scheme, and to force put an oop somewhere.
    // Useful when building the space, for instance.
    pub fn build_oop_at(&self, index: usize, space: &mut MemorySpace) {
        let mut new_oop_carcass = OopCarcass::default();
        new_oop_carcass.set_number_of_slots(self.number_of_slots);
        new_oop_carcass
            .get_header_mut()
            .set_class_index_bits(self.class_index);
        new_oop_carcass.apply_at_index_on_space(index, space);
    }

    pub fn build(&self, space: &mut MemorySpace) -> usize {
        let mut new_oop_carcass = OopCarcass::default();
        new_oop_carcass.set_number_of_slots(self.number_of_slots);
        let new_oop_size = new_oop_carcass.oop_size();

        let allocated_index: usize = where_to_allocate(new_oop_carcass.oop_size(), space);
        let free_header = OopHeaders::new(allocated_index, space);
        let free_oop_size = free_header.oop_size();

        if free_oop_size != new_oop_size {
            // todo update header_size if the new oop isn't big anymore
            let new_free_number_of_slots =
                free_oop_size - new_oop_size - free_header.get_header().header_size();
            let mut new_free_oop = OopCarcass::new_from(&free_header);
            new_free_oop.set_number_of_slots(new_free_number_of_slots);

            let new_free_oop_index: usize = allocated_index + new_oop_size;
            new_free_oop.apply_at_index_on_space(new_free_oop_index, space);
        }

        self.build_oop_at(allocated_index, space);
        allocated_index
    }

    pub fn set_number_of_slots(&mut self, new_number_of_slots: usize) {
        self.number_of_slots = new_number_of_slots;
    }

    pub fn set_class_index(&mut self, new_class_index: usize) {
        self.class_index = new_class_index;
    }
}
