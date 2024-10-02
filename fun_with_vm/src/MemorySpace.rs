struct MemorySpace {
	memory_vector: Vec<usize>
}

impl MemorySpace {

	fn for_bit_size(memory_space_size: usize) -> Self{
		let mut memory_space: Vec<usize> = Vec::with_capacity(memory_space_size);
		for _i in 0..memory_space_size {
			memory_space.push(0);
		}
		//TODO set the first OOP as free
		Self { memory_vector: memory_space }
	}

	fn report(&self){
		println!("memory_vector = {}" , self.memory_vector.len());
	}
}
