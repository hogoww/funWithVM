use crate::header::Header;
use crate::memory_space_iterator::MemorySpaceIterator;
use crate::oop::Oop;
use crate::special_class_index::SpecialClassIndexes;

#[derive(Debug)]
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

        Self {
            memory_vector: memory_space,
        }
    }

    pub fn get_start_index(&self) -> usize {
        0
    }

    pub fn get_end_index(&self) -> usize {
        self.memory_vector.capacity() - 1 // 0 based
    }

    // Beware, This doesn't check that the index hits an header
    pub fn get_oop_at(&mut self, index: usize) -> Oop {
        if index > self.get_end_index() {
            panic!("oop at index {} is out of space bounds", index);
        }

        let header = Header {
            header_value: self[index],
        };
      	
        Oop::new(index, &mut self.memory_vector[index..index + header.oop_size()])
    }

    pub fn iter(&mut self) -> MemorySpaceIterator {
        MemorySpaceIterator::new(self)
    }

    pub fn first_oop(&mut self) -> Oop {
        self.get_oop_at(0)
    }

    pub fn memory_slice(&mut self, start_index: usize, end_index: usize) -> &mut [usize] {
        &mut self.memory_vector[start_index..end_index]
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
        &self.memory_vector[index]
    }
}

use std::ops::IndexMut;

impl IndexMut<usize> for MemorySpace {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory_vector[index]
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::memory_space::MemorySpace;

//     #[test]
//     fn test_unfilled_space_first_oop_is_free() {
//         let space = MemorySpace::for_bit_size(240);
//         assert!(space.first_oop().is_free_oop());
//     }

//     #[test]
//     fn test_unfilled_space_first_oop_is_the_only_oop_in_space() {
//         // The next index will be right after the end of the space
//         let space = MemorySpace::for_bit_size(240);
//         assert_eq!(
//             space.first_oop().next_oop_index() - 1,
//             space.get_end_index()
//         );
//     }
// }
