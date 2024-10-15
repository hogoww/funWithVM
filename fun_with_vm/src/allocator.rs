use crate::memory_space::MemorySpace;
use crate::oop::Oop;

//Remove this mutability...? this function doesn't need to mutate the memory space
pub fn where_to_allocate(number_of_usize: usize , space: &mut MemorySpace) -> usize {
	let mut index : usize = space.get_start_index();
	let last_index = space.get_end_index();

	while index < last_index {
		let oop : Oop = space.get_oop_at(index);
		if oop.is_free_oop() && oop.number_of_slots() >= number_of_usize {
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

#[cfg(test)]
mod tests {
	use crate::memory_space::MemorySpace;
	use crate::oop_builder::OopBuilder;

	#[test]
	fn test_allocate_first_object(){
		let mut space = MemorySpace::for_bit_size(240);
		let builder = OopBuilder::new();
		builder.build(& mut space);

		assert!(! space.first_oop().is_free_oop());
	}

	#[test]
	fn test_allocate_object_that_fit_in_hole(){
		let mut space = MemorySpace::for_bit_size(240);
		let builder = OopBuilder::new();
		builder.build(& mut space);
		builder.build(& mut space);
		space.first_oop().become_free_oop(& mut space);
		let new_object = builder.build(& mut space);
		assert_eq!(new_object, space.get_start_index());
	}

// ==> testAllocateTwoObjectsFillsTheSpace.cpp <==
// #include "memorySpace.hpp"
// #include "cTestCase.h"

// #ifndef WORD_TYPE
// #define WORD_TYPE uint64_t
// #endif

// int main(){
//   MemorySpace<WORD_TYPE> ms(640);
//   OopBuilder<WORD_TYPE>* oopBuilder = ms.getOopBuilder();
//   oopBuilder -> setNumberOfSlots(1);
//   oopBuilder -> build();
  
//   oopBuilder -> reset();
//   oopBuilder -> setNumberOfSlots(2);
//   oopBuilder -> build();
    
//   cAssert(__LINE__, ms.firstOop().nextOop().nextOop().nextOop().getAddress() == ms.getEndAddress());
//   testPassed();
// }

// ==> testAllocateTwoObjectsFirstObjectIsNotOverriden.cpp <==
// #include "memorySpace.hpp"
// #include "cTestCase.h"

// #ifndef WORD_TYPE
// #define WORD_TYPE uint64_t
// #endif

// int main(){
//   MemorySpace<WORD_TYPE> ms(640);
//   OopBuilder<WORD_TYPE>* oopBuilder = ms.getOopBuilder();
//   oopBuilder -> setNumberOfSlots(1);
//   oopBuilder -> build();
  
//   oopBuilder -> reset();
//   oopBuilder -> setNumberOfSlots(2);
//   oopBuilder -> build();

//   cAssertInts(__LINE__, ms.firstOop().getHeader().numberOfSlotsBits(), 1);
//   testPassed();
// }

// ==> testAllocateTwoObjectsHasTwoObjects.cpp <==
// #include "memorySpace.hpp"
// #include "cTestCase.h"

// #ifndef WORD_TYPE
// #define WORD_TYPE uint64_t
// #endif

// int main(){
//   MemorySpace<WORD_TYPE> ms(640);
//   OopBuilder<WORD_TYPE>* oopBuilder = ms.getOopBuilder();
//   oopBuilder -> setNumberOfSlots(1);
//   oopBuilder -> build();
  
//   oopBuilder -> reset();
//   oopBuilder -> setNumberOfSlots(2);
//   oopBuilder -> build();
    
//   cAssert(__LINE__, ms.firstOop().nextOop().nextOop().isFreeOop());
//   testPassed();
// }

// ==> testAllocateTwoObjectsSecondObjectHasCorrectNumberOfSlots.cpp <==
// #include "memorySpace.hpp"
// #include "cTestCase.h"

// #ifndef WORD_TYPE
// #define WORD_TYPE uint64_t
// #endif

// int main(){
//   MemorySpace<WORD_TYPE> ms(640);
//   OopBuilder<WORD_TYPE>* oopBuilder = ms.getOopBuilder();
//   oopBuilder -> setNumberOfSlots(1);
//   oopBuilder -> build();
  
//   oopBuilder -> reset();
//   oopBuilder -> setNumberOfSlots(2);
//   oopBuilder -> build();
    
//   cAssertInts(__LINE__, ms.firstOop().nextOop().getHeader().numberOfSlotsBits(), 2);
//   testPassed();
// }

// ==> testAllocateTwoObjectsSecondObjectIsNotFree.cpp <==
// #include "memorySpace.hpp"
// #include "cTestCase.h"

// #ifndef WORD_TYPE
// #define WORD_TYPE uint64_t
// #endif

// int main(){
//   MemorySpace<WORD_TYPE> ms(640);
//   OopBuilder<WORD_TYPE>* oopBuilder = ms.getOopBuilder();
//   oopBuilder -> setNumberOfSlots(1);
//   oopBuilder -> build();
  
//   oopBuilder -> reset();
//   oopBuilder -> setNumberOfSlots(2);
//   oopBuilder -> build();
    
//   cAssert(__LINE__, not ms.firstOop().nextOop().isFreeOop() );
//   testPassed();
// }

// ==> testMovedFreeObjectHasLessSlots.cpp <==
// #include "memorySpace.hpp"
// #include "cTestCase.h"

// #ifndef WORD_TYPE
// #define WORD_TYPE uint64_t
// #endif

// int main(){
//   MemorySpace<WORD_TYPE> ms(640);
//   OopBuilder<WORD_TYPE>* oopBuilder = ms.getOopBuilder();
//   oopBuilder -> setNumberOfSlots(1);
//   oopBuilder -> build();
    
//   cAssertInts(__LINE__, ms.firstOop().nextOop().getHeader().numberOfSlotsBits(), ms.wordSpaceSize() - 2 - 1);// 2 == oop header + slot, 1 == freeOopheader
//   testPassed();
// }

// ==> testMovedFreeObjectIsAfterFirst.cpp <==
// #include "memorySpace.hpp"
// #include "cTestCase.h"

// #ifndef WORD_TYPE
// #define WORD_TYPE uint64_t
// #endif

// int main(){
//   MemorySpace<WORD_TYPE> ms(640);
//   OopBuilder<WORD_TYPE>* oopBuilder = ms.getOopBuilder();
//   oopBuilder -> setNumberOfSlots(1);
//   oopBuilder -> build();
    
//   cAssert(__LINE__, ms.firstOop().nextOop().isFreeOop());
//   testPassed();
// }

}
