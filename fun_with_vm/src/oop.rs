use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::special_class_index::SpecialClassIndexes;

pub struct Oop {
    index: usize,
    contents: Vec<usize>,
}

impl Oop {
    pub fn new(index: usize, contents: Vec<usize>) -> Self {
        return Self {
            index: index,
            contents: contents,
        };
    }

    fn header_index(&self) -> usize {
        return 0;
    }

    pub fn set_header(&mut self, header: Header) {
        let index: usize = self.header_index();
        self.contents[index] = header.get_value();
    }

    pub fn get_header(&self) -> Header {
        return Header {
            header_value: self.header_value(),
        };
    }

    //shortcut
    pub fn header_value(&self) -> usize {
        return self.contents[self.header_index()];
    }

    pub fn get_index(&self) -> usize {
        return self.index;
    }

    pub fn is_free_oop(&self) -> bool {
        return self.get_header().class_index_bits() == SpecialClassIndexes::FreeObject as usize;
    }

    pub fn become_free_oop(&mut self, space: &mut MemorySpace) {
        let mut header = self.get_header();
        header.set_class_index_bits(SpecialClassIndexes::FreeObject as usize);
        let header_index = self.header_index();
        self.contents[header_index] = header.header_value;
        self.apply_to_space(space);
    }

    pub fn apply_to_space(&mut self, space: &mut MemorySpace) {
        let mut index = self.index;
        for value in &self.contents {
            space[index] = *value;
            index = index + 1;
        }
    }

    pub fn next_oop_index(&self) -> usize {
        return self.index + self.get_header().oop_size();
    }

    pub fn next_oop(&self, space: &MemorySpace) -> Oop {
        return space.get_oop_at(self.next_oop_index());
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
        return self.get_header().number_of_slots_bits();
    }

    fn slot_bound_check(&self, an_index: usize) -> bool {
        return an_index == 0 || an_index > self.number_of_slots();
    }

    pub fn slot_at_index(&self, an_index: usize) -> usize {
        if self.slot_bound_check(an_index) { //exit(1);
        }
        return self.contents[an_index];
    }

    pub fn slot_at_put(&mut self, an_index: usize, an_oop_address: usize) {
        if self.slot_bound_check(an_index) {} // exit(1);
        let slot_index = self.header_index() + an_index;
        self.contents[slot_index] = an_oop_address;
    }
}

#[cfg(test)]
mod tests {
    use crate::memory_space::MemorySpace;
    use crate::oop_builder::OopBuilder;
	use crate::oop::Oop;

    #[test]
    fn become_free_oop_is_free_oop() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let oop_index = builder.build(&mut space);
        let mut new_object = space.get_oop_at(oop_index);

        new_object.become_free_oop(&mut space);
        assert!(new_object.is_free_oop());
    }

	#[test]
	fn test_slot_at_index_returns_value() {
        let mut space = MemorySpace::for_bit_size(240);
        let mut builder = OopBuilder::new();
		builder.set_number_of_slots(1);
		let oop_index : usize = builder.build(&mut space);
		let slot_index : usize = 1;
		let slot_value : usize = 3;
		space[oop_index + slot_index] = slot_value;

		let oop : Oop = space.first_oop();
        assert_eq!(oop.slot_at_index(1), slot_value);
	}
	
	#[test]
	fn test_slot_at_put_sets_value() {
        let mut space = MemorySpace::for_bit_size(240);
        let mut builder = OopBuilder::new();
		builder.set_number_of_slots(1);
		builder.build(&mut space);
		let slot_index : usize = 1;
		let slot_value : usize = 3;

		let mut oop : Oop = space.first_oop();
		oop.slot_at_put(slot_index,slot_value);
        assert_eq!(oop.slot_at_index(1), slot_value);
	}
}