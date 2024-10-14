use crate::header::Header;
use crate::allocator::where_to_allocate;
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
		return Self { class_index: 2, number_of_slots: 0 };
	}
	
	pub fn initialize(&mut self){
		//The class index should probably be different than existing classes (i.e., freeObject)
		//Maybe it should always be required
		self.class_index = 2;
		self.number_of_slots = 0;
	}
	
	// API, for code readability
	pub fn reset(&mut self){
		self.initialize();
	}
	
	pub fn build(&self, space: & mut MemorySpace) -> usize {
		// Need to pass the space as a muttable, but this is unrequired by rust ?
		let allocation_index = where_to_allocate(self.number_of_slots, space);
		let new_oop_size = self.number_of_slots + 1; // header_size
		let new_free_oop_index = allocation_index + new_oop_size;
		let mut oop_header = Header { header_value: 0 };
		let mut free_header = Header { header_value: space[ allocation_index ] };
		free_header.set_number_of_slots_bits(free_header.number_of_slots_bits() - new_oop_size);
		free_header.set_class_index_bits(SpecialClassIndexes::FreeObject as usize);

		oop_header.set_number_of_slots_bits(self.number_of_slots);
		oop_header.set_class_index_bits(self.class_index);
		//oop_header.set_format_bits(self.format);

		space[allocation_index] = oop_header.header_value;
		space[new_free_oop_index] = free_header.header_value;
		return allocation_index;
	}

	pub fn set_number_of_slots(&mut self, new_number_of_slots: usize){
		self.number_of_slots = new_number_of_slots;
	}

	pub fn set_class_index(&mut self, new_class_index: usize){
		self.class_index = new_class_index;
	}
}

