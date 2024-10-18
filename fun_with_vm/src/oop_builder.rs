use crate::allocator::where_to_allocate;
use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::special_class_index::SpecialClassIndexes;

#[derive(Default)]
pub struct OopBuilder {
    //slots
    number_of_slots: usize,
    class_index: usize,
}

impl OopBuilder {
    pub fn new() -> OopBuilder {
        return Self {
            class_index: 2,
            number_of_slots: 0,
        };
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

    pub fn build(&self, space: &mut MemorySpace) -> usize {
        let allocation_index: usize = where_to_allocate(self.number_of_slots, &space);
        let new_oop_size: usize = self.number_of_slots + 1; // header_size
        let new_free_oop_index: usize = allocation_index + new_oop_size;
        let mut oop_header = Header { header_value: 0 };
        let mut free_header = Header {
            header_value: space[allocation_index],
        };

        oop_header.set_number_of_slots_bits(self.number_of_slots);
        oop_header.set_class_index_bits(self.class_index);

        let new_free_number_of_slots = free_header
            .number_of_slots_bits()
            .overflowing_sub(oop_header.oop_size());

        if !new_free_number_of_slots.1 {
            free_header.set_number_of_slots_bits(new_free_number_of_slots.0);

            free_header.set_class_index_bits(SpecialClassIndexes::FreeObject as usize);
            space[new_free_oop_index] = free_header.header_value;
        }

        space[allocation_index] = oop_header.header_value;
        return allocation_index;
    }

    pub fn set_number_of_slots(&mut self, new_number_of_slots: usize) {
        self.number_of_slots = new_number_of_slots;
    }

    pub fn set_class_index(&mut self, new_class_index: usize) {
        self.class_index = new_class_index;
    }
}
