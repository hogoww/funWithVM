mod simple_garbage_collector {
    use crate::memory_space::MemorySpace;
    use crate::oop::Oop;

    #[allow(dead_code)]
    pub fn collect_from_roots(roots: Vec<usize>, space: &mut MemorySpace) {
        mark_oops_from_roots(roots, space);
        //   this -> sweepOops();
        //   this -> mergeFreeOops();
    }

    #[allow(dead_code)]
    pub fn mark_oops_from_roots(roots: Vec<usize>, space: &mut MemorySpace) {
        let mut oop_to_mark: Vec<usize> = roots.clone();

        while let Some(an_oop_index) = oop_to_mark.pop() {
            let mut an_oop: Oop = space.get_oop_at(an_oop_index);
            if an_oop.get_header().marked_bit() != 1 {
                //println!("Marking {}", an_oop_index);

                an_oop.get_header().set_marked_bit();
                an_oop.apply_header(space);

                //TODO(slots)FLAG should NOT iterate over slots. This is the Oop responsibility
                let number_of_slots: usize = an_oop.get_header().number_of_slots_bits();
                for index in 1..=number_of_slots {
                    oop_to_mark.push(an_oop.slot_at_index(index));
                }
            }
        }
    }

    // template <typename WORD_TYPE>
    // void GarbageCollector<WORD_TYPE>::sweepOops(){
    //   Oop<WORD_TYPE> currentOop = memorySpace -> firstOop();
    //   while ( currentOop.getAddress() < memorySpace -> getEndAddress() ){
    //     if(currentOop.getHeader().markedBit()){
    //       currentOop.getHeader().unsetMarkedBit();
    //     }
    //     else {
    //       currentOop.becomeFreeOop();
    //     }

    //     currentOop = currentOop.nextOop();
    //   }
    // }

    // template <typename WORD_TYPE>
    // void GarbageCollector<WORD_TYPE>::mergeFreeOops(){
    //   WORD_TYPE* endAddress = memorySpace -> getEndAddress();
    //   Oop<WORD_TYPE> currentOop = memorySpace -> firstOop();
    //   Oop<WORD_TYPE> nextOop = currentOop.nextOop();

    //   while ( currentOop.getAddress() < endAddress ){
    //     if(currentOop.isFreeOop() && nextOop.getAddress() < endAddress && nextOop.isFreeOop()){
    //       // + 1 because the header has the same size as a slot (at this time)
    //       currentOop.getHeader().setNumberOfSlotsBits(currentOop.getHeader().numberOfSlotsBits() + nextOop.getHeader().numberOfSlotsBits() + 1);
    //     }
    //     else {
    //       currentOop = currentOop.nextOop();
    //     }
    //     nextOop = currentOop.nextOop();

    //   }
    // }
}

#[cfg(test)]
mod tests {
    use crate::garbage_collector::simple_garbage_collector;
    use crate::memory_space::MemorySpace;
    use crate::oop_builder::OopBuilder;

    // Marking tests
    #[test]
    fn test_mark_roots() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let mut roots: Vec<usize> = Vec::new();
        roots.push(builder.build(&mut space));

        simple_garbage_collector::mark_oops_from_roots(roots, &mut space);

        assert_eq!(space.first_oop().get_header().marked_bit(), 1);
    }

    #[test]
    fn test_mark_slot_of_root() {
        let mut space = MemorySpace::for_bit_size(240);
        let mut builder = OopBuilder::new();
        let mut roots: Vec<usize> = Vec::new();
        builder.set_number_of_slots(1);
        roots.push(builder.build(&mut space));
        builder.reset();
        let mut first_oop = space.first_oop();
        first_oop.slot_at_put(1, builder.build(&mut space));
        first_oop.apply_to_space(&mut space);

        simple_garbage_collector::mark_oops_from_roots(roots, &mut space);

        assert_eq!(
            space
                .first_oop()
                .next_oop(&mut space)
                .get_header()
                .marked_bit(),
            1
        );
    }

    #[test]
    fn test_sweep_clears_market_bit() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let mut roots: Vec<usize> = Vec::new();
        roots.push(builder.build(&mut space));

        simple_garbage_collector::collect_from_roots(roots, &mut space);

        assert_eq!(space.first_oop().get_header().marked_bit(), 0);
    }

    // Regular test
    #[test]
    fn test_garbage_collection_creates_hole() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let mut roots: Vec<usize> = Vec::new();
        builder.build(&mut space);
        roots.push(builder.build(&mut space));

        simple_garbage_collector::collect_from_roots(roots, &mut space);

        assert!(space.first_oop().is_free_oop());
    }

    #[test]
    fn test_garbage_collection_does_not_reclaim_roots() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let mut roots: Vec<usize> = Vec::new();
        roots.push(builder.build(&mut space));

        simple_garbage_collector::collect_from_roots(roots, &mut space);

        assert!(!space.first_oop().is_free_oop());
    }

    #[test]
    fn test_garbage_collection_does_not_reclaim_slot_of_root() {
        let mut space = MemorySpace::for_bit_size(240);
        let mut builder = OopBuilder::new();
        let mut roots: Vec<usize> = Vec::new();
        builder.set_number_of_slots(1);
        roots.push(builder.build(&mut space));
        builder.reset();
        space.first_oop().slot_at_put(1, builder.build(&mut space));

        simple_garbage_collector::collect_from_roots(roots, &mut space);

        assert!(!space.first_oop().next_oop(&mut space).is_free_oop());
    }

    #[test]
    fn test_garbage_collection_reclaims_all_objects_without_roots() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let roots: Vec<usize> = Vec::new();
        builder.build(&mut space);

        simple_garbage_collector::collect_from_roots(roots, &mut space);

        assert_eq!(space.first_oop().number_of_slots(), 239);
    }

    //compaction tests
    #[test]
    fn test_garbage_collection_compacts_free_oop_reclaimed_after_a_free_oop() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let roots: Vec<usize> = Vec::new();
        builder.build(&mut space);
        builder.build(&mut space);
        space.first_oop().become_free_oop(&mut space);

        simple_garbage_collector::collect_from_roots(roots, &mut space);

        assert_eq!(space.first_oop().number_of_slots(), 238);
    }

    #[test]
    fn test_garbage_collection_compacts_free_oop_reclaimed_before_a_free_oop() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let roots: Vec<usize> = Vec::new();
        builder.build(&mut space);
        space.first_oop().become_free_oop(&mut space);

        simple_garbage_collector::collect_from_roots(roots, &mut space);

        assert_eq!(space.first_oop().number_of_slots(), 238);
    }
}
