use crate::header::Header;
use crate::oop::Oop;
use crate::special_class_index::SpecialClassIndexes;

pub struct MemorySpace {
    memory_vector: Vec<usize>,
}

impl MemorySpace {
    pub fn for_bit_size(memory_space_size: usize) -> Self {
        let mut memory_space: Vec<usize> = vec![0; memory_space_size];

        // set first oop to be free & have all the slots in the space
        let mut free_oop_header = Header { header_value: 0 };
        free_oop_header.set_class_index_bits(SpecialClassIndexes::FreeObject as usize);
        //TODO support spaces biggers than 256*sizeof(usize).
        free_oop_header.set_number_of_slots_bits(memory_space_size - 1); // minus the header for the space
        memory_space[0] = free_oop_header.header_value;

        return Self {
            memory_vector: memory_space,
        };
    }

    pub fn get_start_index(&self) -> usize {
        return 0;
    }

    pub fn get_end_index(&self) -> usize {
        return self.memory_vector.capacity() - 1; // 0 based
    }

    // Beware, no check that the index is correct
    pub fn get_oop_at(&self, index: usize) -> Oop {
        let header = Header {
            header_value: self[index],
        };
        let mut oop_content: Vec<usize> = vec![0; header.oop_size()];
        oop_content.copy_from_slice(&self.memory_vector[index..index + header.oop_size()]);
        return Oop::new(index, oop_content);
    }

    pub fn first_oop(&self) -> Oop {
        return self.get_oop_at(0);
    }

    // pub fn setIndexToValue(&mut self, index: usize , value: usize){
    // 	self.memory_vector[index] = value
    // }

    pub fn report(&self) {
        println!("memory_vector = {}", self.memory_vector.len());
    }
}

use std::ops::Index;

impl Index<usize> for MemorySpace {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.memory_vector[index];
    }
}

use std::ops::IndexMut;

impl IndexMut<usize> for MemorySpace {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.memory_vector[index];
    }
}
