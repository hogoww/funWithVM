
use crate::header::Header;
use crate::special_class_index::SpecialClassIndexes;


pub struct MemorySpace {
	memory_vector: Vec<usize>,
}

impl MemorySpace {

	pub fn for_bit_size(memory_space_size: usize) -> Self {
		let mut memory_space: Vec<usize> = Vec::with_capacity(memory_space_size);
		//need to allocate the memory, since it's a vector, one case at a time.
		for _i in 0..memory_space_size {
			memory_space.push(0);
		}

		// set first oop to be free & have all the slots in the space
		let mut free_oop_header = Header{ header_value: 0 };
		free_oop_header.set_class_index_bits(SpecialClassIndexes::FreeObject as usize);
		free_oop_header.set_number_of_slots_bits(memory_space_size);
		memory_space[0] = free_oop_header.header_value;
		
		return Self { memory_vector: memory_space };
	}

	// pub fn setIndexToValue(&mut self, index: usize , value: usize){
	// 	self.memory_vector[index] = value
	// }
	
	pub fn report(&self){
		println!("memory_vector = {}" , self.memory_vector.len());
	}
}

use std::ops::Index;

impl Index<usize> for MemorySpace{
	type Output = usize;
	
	fn index(&self, index: usize) -> &Self::Output {
		return &self.memory_vector[index];
	}
}

use std::ops::IndexMut;

impl IndexMut<usize> for MemorySpace {
	fn index_mut(&mut self, index: usize) -> & mut Self::Output {
		return &mut self.memory_vector[index];
	}
}
