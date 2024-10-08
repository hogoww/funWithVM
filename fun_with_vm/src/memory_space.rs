pub struct MemorySpace {
	memory_vector: Vec<usize>
}

impl MemorySpace {

	pub fn for_bit_size(memory_space_size: usize) -> Self{
		let mut memory_space: Vec<usize> = Vec::with_capacity(memory_space_size);
		for _i in 0..memory_space_size {
			memory_space.push(0);
		}
		//TODO set the first OOP as free
		Self { memory_vector: memory_space }
	}

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
