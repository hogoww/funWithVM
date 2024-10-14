//use crate::memory_space::MemorySpace;
use crate::header::Header;
use crate::special_class_index::SpecialClassIndexes;

pub struct Oop {
	index: usize,
	contents: Vec<usize>,
}


impl Oop {
	pub fn new(index: usize, contents: Vec<usize>) -> Self {
		return Self { index: index, contents: contents };
	}
	
	fn header_index(&self) -> usize {
		return 0;
	}
	
	pub fn set_header(&mut self, header: Header) {
		let index : usize = self.header_index();
		self.contents[index] = header.get_value();
	}

	pub fn get_header(&self) -> Header {
		return Header { header_value: self.header_value() };
	}

	//shortcut
	pub fn header_value(&self) -> usize {
		return self.contents[self.header_index()]
	}

	pub fn get_index(&self) -> usize {
		return self.index;
	}

	pub fn is_free_oop(&self) -> bool {
		return self.get_header().class_index_bits() == SpecialClassIndexes::FreeObject as usize;
	}

// template <typename WORD_TYPE>
// void Oop<WORD_TYPE>::becomeFreeOop(){
//   this -> header.setClassIndexBits(specialClassIndexes::freeObject);
// }

	pub fn next_oop_index(&self) -> usize {
		return self.index + self.get_header().number_of_slots_bits();
	}

// template <typename WORD_TYPE>
// WORD_TYPE Oop<WORD_TYPE>::bitSize(){
//   return this -> header.bitSize();
// }

// template <typename WORD_TYPE>
// WORD_TYPE Oop<WORD_TYPE>::wordSize(){
//   return this -> header.wordSize();
// }

	pub fn number_of_slots(&self) -> usize {
		return self.get_header().number_of_slots_bits()
	}
	
	fn slot_bound_check(&self, an_index:usize) -> bool {
		return an_index == 0 || an_index > self.number_of_slots();
	}
	
	pub fn slot_at_index(&self, an_index: usize) -> usize {
		if self.slot_bound_check(an_index)
		{ //exit(1);
		}
		return self.index + an_index;
	}

	pub fn slot_at_put(&mut self, an_index: usize, an_oop_address: usize){
		if self.slot_bound_check(an_index)
		{}// exit(1);
		let slot_index = self.header_index() + an_index;
		self.contents[slot_index] = an_oop_address;
	}
}

#[cfg(test)]
mod tests {
//	use crate::Oop;

	// int main(){
	//   MemorySpace<WORD_TYPE> ms(640);
	//   OopBuilder<WORD_TYPE>* oopBuilder = ms.getOopBuilder();
	//   oopBuilder -> setNumberOfSlots(1);
	//   oopBuilder -> build();
	//   oopBuilder -> build();
	
	//   Oop<WORD_TYPE> container = ms.firstOop();
	//   container.slotAtPut( 1 , container.nextOop().getAddress());
	
	//   cAssert(__LINE__, container.slotAt(1).getAddress() == ms.firstOop().nextOop().getAddress());
	// }
}
