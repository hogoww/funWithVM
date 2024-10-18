mod garbage_collector {
    use crate::memory_space::MemorySpace;
    //use crate::oop::Oop;

    pub fn collect_from_roots(_roots: Vec<usize>, _space: &mut MemorySpace) {
        //todo !
    }

    pub fn mark_oops_from_roots(_roots: Vec<usize>, _space: &mut MemorySpace) {
        //todo !
    }
}

#[cfg(test)]
mod tests {
    use crate::garbage_collector::garbage_collector;
    use crate::memory_space::MemorySpace;
    use crate::oop_builder::OopBuilder;

    // Marking tests
    #[test]
    fn test_mark_roots() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let mut roots: Vec<usize> = Vec::new();
        roots.push(builder.build(&mut space));

        garbage_collector::mark_oops_from_roots(roots, &mut space);

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
        space.first_oop().slot_at_put(1, builder.build(&mut space));

        garbage_collector::mark_oops_from_roots(roots, &mut space);

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

        garbage_collector::collect_from_roots(roots, &mut space);

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

        garbage_collector::collect_from_roots(roots, &mut space);

        assert!(space.first_oop().is_free_oop());
    }

    #[test]
    fn test_garbage_collection_does_not_reclaim_roots() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let mut roots: Vec<usize> = Vec::new();
        roots.push(builder.build(&mut space));

        garbage_collector::collect_from_roots(roots, &mut space);

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

        garbage_collector::collect_from_roots(roots, &mut space);

        assert!(!space.first_oop().next_oop(&mut space).is_free_oop());
    }

    #[test]
    fn test_garbage_collection_reclaims_all_objects_without_roots() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let roots: Vec<usize> = Vec::new();
        builder.build(&mut space);

        garbage_collector::collect_from_roots(roots, &mut space);

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

        garbage_collector::collect_from_roots(roots, &mut space);

        assert_eq!(space.first_oop().number_of_slots(), 238);
    }

    #[test]
    fn test_garbage_collection_compacts_free_oop_reclaimed_before_a_free_oop() {
        let mut space = MemorySpace::for_bit_size(240);
        let builder = OopBuilder::new();
        let roots: Vec<usize> = Vec::new();
        builder.build(&mut space);
        space.first_oop().become_free_oop(&mut space);

        garbage_collector::collect_from_roots(roots, &mut space);

        assert_eq!(space.first_oop().number_of_slots(), 238);
    }
}
