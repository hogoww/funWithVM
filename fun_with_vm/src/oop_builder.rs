use crate::allocator::where_to_allocate;
use crate::header::Header;
use crate::memory_space::MemorySpace;
//use crate::oop::*;
use crate::special_class_index::SpecialClassIndexes;

#[derive(Default)]
pub struct OopBuilder {
    //slots
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

    pub fn build(&self, space: &mut MemorySpace) -> usize {
        let mut oop_header = Header::new();

        if self.number_of_slots > Header::MAX_NUMBER_OF_SLOTS {
            oop_header.set_number_of_slots_to_max();
        } else {
            oop_header.set_number_of_slots_bits(self.number_of_slots);
        }

        let allocated_index: usize =
            where_to_allocate(oop_header.header_size() + self.number_of_slots, space);

        let mut free_header = Header {
            header_value: space[allocated_index],
        };

        let free_oop_size = space.oop_size_at(allocated_index);
        let new_oop_size = oop_header.header_size() + self.number_of_slots;

        if free_oop_size != new_oop_size {
            //TODO(big oop)
            let new_free_number_of_slots = free_oop_size - new_oop_size - free_header.header_size();
            if new_free_number_of_slots > Header::MAX_NUMBER_OF_SLOTS {
                free_header.set_number_of_slots_to_max();
            } else {
                free_header.set_number_of_slots_bits(self.number_of_slots);
            }
            free_header.set_number_of_slots_bits(new_free_number_of_slots);

            free_header.set_class_index_bits(SpecialClassIndexes::FreeObject as usize);
            let new_free_oop_index: usize = allocated_index + new_oop_size;
            space[new_free_oop_index] = free_header.header_value;
            if free_header.has_extra_slot_header() {
                space[new_free_oop_index + 1] = self.number_of_slots;
            }
        }

        oop_header.set_class_index_bits(self.class_index);
        space[allocated_index] = oop_header.header_value;
        if oop_header.has_extra_slot_header() {
            space[allocated_index + 1] = self.number_of_slots;
        }

        allocated_index
    }

    pub fn set_number_of_slots(&mut self, new_number_of_slots: usize) {
        self.number_of_slots = new_number_of_slots;
    }

    pub fn set_class_index(&mut self, new_class_index: usize) {
        self.class_index = new_class_index;
    }
}
