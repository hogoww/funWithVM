use crate::memory_space::MemorySpace;
use crate::oop::Oop;

pub fn where_to_allocate(number_of_usize: usize, space: &mut MemorySpace) -> usize {
    let mut index: usize = space.get_start_index();
    let last_index = space.get_end_index();

    while index < last_index {
        let oop: Oop = space.get_oop_at(index);
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
    fn test_allocate_first_object() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        builder.build(&mut space);

        assert!(!space.iter().next().unwrap().is_free_oop());
    }

    // #[test]
    // fn test_allocate_object_that_fit_in_hole() {
    //     let mut space = MemorySpace::for_bit_size(240);
    //     let builder = OopBuilder::new();
    //     builder.build(&mut space);
    //     builder.build(&mut space);
    //     space.first_oop().become_free_oop();
    //     let new_object = builder.build(&mut space);
    //     assert_eq!(new_object, space.get_start_index());
    // }

    // #[test]
    // fn test_allocate_big_object_fills_the_space() {
    //     let mut space = MemorySpace::for_bit_size(240);
    //     let mut builder = OopBuilder::new();
    //     builder.set_number_of_slots(239);
    //     builder.build(&mut space);
    //     println!(
    //         "{}, {}",
    //         space.first_oop().next_oop_index(),
    //         space.get_end_index()
    //     );
    //     assert!(space.first_oop().next_oop_index() > space.get_end_index());
    // }

    // #[test]
    // fn test_allocate_two_objects_first_object_is_not_overriden() {
    //     let mut space = MemorySpace::for_bit_size(240);
    //     let mut builder = OopBuilder::new();
    //     builder.set_number_of_slots(1);
    //     builder.build(&mut space);
    //     builder.reset();
    //     builder.set_number_of_slots(2);
    //     builder.build(&mut space);

    //     assert_eq!(space.first_oop().get_header().number_of_slots_bits(), 1);
    // }

    // #[test]
    // fn test_allocate_two_objects_has_two_objects() {
    //     let mut space = MemorySpace::for_bit_size(240);
    //     let mut builder = OopBuilder::new();
    //     builder.set_number_of_slots(1);
    //     builder.build(&mut space);
    //     builder.reset();
    //     builder.set_number_of_slots(2);
    //     builder.build(&mut space);

	// 	let iter = space.iter();
	// 	iter.next();
	// 	iter.next();
    //     assert!(iter.next().unwrap().is_free_oop());
	// }

    // #[test]
    // fn test_allocate_two_objects_second_object_has_correct_number_of_slots() {
    //     let mut space = MemorySpace::for_bit_size(240);
    //     let mut builder = OopBuilder::new();
    //     builder.set_number_of_slots(1);
    //     builder.build(&mut space);
    //     builder.reset();
    //     builder.set_number_of_slots(2);
    //     builder.build(&mut space);

	// 	let iter = space.iter();
	// 	iter.next();

    //     assert_eq!(
	// 		iter.next().unwrap().get_header().number_of_slots_bits(),
    //         2
    //     );
    // }

    // #[test]
    // fn test_allocate_two_objects_second_object_is_not_free() {
    //     let mut space = MemorySpace::for_bit_size(240);
    //     let mut builder = OopBuilder::new();
    //     builder.set_number_of_slots(1);
    //     builder.build(&mut space);
    //     builder.reset();
    //     builder.set_number_of_slots(2);
    //     builder.build(&mut space);

	// 	let iter = space.iter();
	// 	iter.next();
		
    //     assert!(!iter.next().unwrap().is_free_oop());
    // }

    // #[test]
    // fn test_moved_free_object_has_less_slots() {
    //     let mut space = MemorySpace::for_bit_size(240);
    //     let mut builder = OopBuilder::new();
    //     builder.set_number_of_slots(1);
    //     builder.build(&mut space);

    //     assert_eq!(
    //         space
    //             .first_oop()
    //             .next_oop(&space)
    //             .get_header()
    //             .number_of_slots_bits(),
    //         237
    //     ); // 2 == oop (header + slot) & 0 based
    // }

    // #[test]
    // fn test_moved_free_object_is_after_first() {
    //     let mut space = MemorySpace::for_bit_size(240);
    //     let mut builder = OopBuilder::new();
    //     builder.set_number_of_slots(1);
    //     builder.build(&mut space);

    //     assert!(space.first_oop().next_oop(&space).is_free_oop());
    // }
}
