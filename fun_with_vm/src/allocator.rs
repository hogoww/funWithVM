use crate::memory_space::MemorySpace;
use crate::oop::Oop;

//Remove this mutability...? this function doesn't need to mutate the memory space
pub fn where_to_allocate(number_of_usize: usize , space: &mut MemorySpace) -> usize {
	let mut index : usize = space.get_start_index();
	let last_index = space.get_end_index();
	while index < last_index {
		let oop : Oop = space.get_oop_at(index);
		if oop.is_free() && oop.number_of_slots() > number_of_usize {
			// We found a free index with enough space !
			return index;
		}
		index = oop.next_oop_index();
	}
	//We didn't find a proper place in memory to put that many usize
	
	// should probably say we need a GC.
	// Maybe thrown an eror in the meantime.
	std::process::exit(1);
}

