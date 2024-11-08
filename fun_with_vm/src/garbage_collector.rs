mod simple_garbage_collector {
    use crate::header::Header;
    use crate::memory_space::MemorySpace;
    use crate::oop::Oop;
    use crate::slot_content::SlotContent;

    #[allow(dead_code)]
    pub fn collect_from_roots(roots: Vec<usize>, space: &mut MemorySpace) {
        mark_oops_from_roots(roots, space);
        sweep_oops(space);
        merge_free_oops(space);
    }

    pub fn mark_oops_from_roots(roots: Vec<usize>, space: &mut MemorySpace) {
        let mut oop_to_mark: Vec<usize> = roots.clone();

        while let Some(an_oop_index) = oop_to_mark.pop() {
            let mut an_oop: Oop = space.get_oop_at(an_oop_index);
            if an_oop.get_header().marked_bit() != 1 {
                //println!("Marking {}", an_oop_index);

                an_oop.get_header().set_marked_bit();
                an_oop.apply_header();

                //TODO(slots)
                // FLAG should NOT iterate over slots. This is the Oop responsibility
                let number_of_slots: usize = an_oop.get_header().number_of_slots_bits();
                for index in 1..=number_of_slots {
                    let slot_content = SlotContent::new(an_oop.slot_at_index(index));
                    if slot_content.is_slot_oop() {
                        oop_to_mark.push(slot_content.get_content());
                    }
                }
            }
        }
    }

    pub fn sweep_oops(space: &mut MemorySpace) {
        let mut iter = space.iter();
        while let Some(mut current_oop) = iter.next(space) {
            if current_oop.get_header().marked_bit() == 1 {
                current_oop.get_header().unset_marked_bit();
            } else {
                current_oop.become_free_oop();
            }
            current_oop.apply_header();
        }
    }

    pub fn merge_free_oops(space: &mut MemorySpace) {
        let last_index = space.get_end_index();
        let mut current_oop_index = space.get_start_index();
        let mut current_oop_header = Header {
            header_value: space[current_oop_index],
        };
        let mut next_oop_index = current_oop_index + space.oop_size_at(current_oop_index);
        let mut next_oop_header;
        //TODO(big oop)
        while next_oop_index < last_index {
            next_oop_header = Header {
                header_value: space[next_oop_index],
            };
            if current_oop_header.is_free_oop() && next_oop_header.is_free_oop() {
                // Merged oops only need one header !
                let new_number_of_slots =
                    current_oop_header.number_of_slots_bits() + space.oop_size_at(next_oop_index);
                current_oop_header.set_number_of_slots_bits(new_number_of_slots);
                space[current_oop_index] = current_oop_header.header_value;
            } else {
                current_oop_index = next_oop_index;
                current_oop_header = Header {
                    header_value: space[current_oop_index],
                };
            }
            next_oop_index = current_oop_index + space.oop_size_at(current_oop_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::garbage_collector::simple_garbage_collector;
    use crate::memory_space::MemorySpace;
    use crate::oop_builder::OopBuilder;

    mod mark_tests {
        use super::*;

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
            let second_oop = builder.build(&mut space);
            let mut first_oop = space.first_oop();
            first_oop.slot_at_put(1, second_oop);

            simple_garbage_collector::mark_oops_from_roots(roots, &mut space);

            let mut iter = space.iter();
            iter.next(&mut space);

            assert_eq!(iter.next(&mut space).unwrap().get_header().marked_bit(), 1);
        }

        #[test]
        fn test_sweep_clears_marked_bit() {
            let mut space = MemorySpace::for_bit_size(240);
            let builder = OopBuilder::new();
            let mut roots: Vec<usize> = Vec::new();
            roots.push(builder.build(&mut space));

            simple_garbage_collector::collect_from_roots(roots, &mut space);

            let mut iter = space.iter();

            assert_eq!(iter.next(&mut space).unwrap().get_header().marked_bit(), 0);
        }
    }

    mod sweep_tests {
        use super::*;

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
            let second_oop = builder.build(&mut space);
            let mut first_oop = space.first_oop();
            first_oop.slot_at_put(1, second_oop);

            simple_garbage_collector::collect_from_roots(roots, &mut space);

            let mut iter = space.iter();
            iter.next(&mut space);
            assert!(!iter.next(&mut space).unwrap().is_free_oop());
        }

        #[test]
        fn test_garbage_collection_reclaims_all_objects_without_roots() {
            let mut space = MemorySpace::for_bit_size(240);
            let builder = OopBuilder::new();
            let roots: Vec<usize> = Vec::new();
            builder.build(&mut space);

            simple_garbage_collector::collect_from_roots(roots, &mut space);

            assert!(space.first_oop().is_free_oop());
        }
    }

    mod merging_tests {
        use super::*;

        #[test]
        fn test_garbage_collection_compacts_free_oop_reclaimed_after_a_free_oop() {
            let mut space = MemorySpace::for_bit_size(240);
            let builder = OopBuilder::new();
            let roots: Vec<usize> = Vec::new();
            builder.build(&mut space);
            builder.build(&mut space);
            space.first_oop().become_free_oop();

            simple_garbage_collector::collect_from_roots(roots, &mut space);

            assert_eq!(space.first_oop().number_of_slots(), 239);
        }

        #[test]
        fn test_garbage_collection_compacts_free_oop_reclaimed_before_a_free_oop() {
            let mut space = MemorySpace::for_bit_size(240);
            let builder = OopBuilder::new();
            let roots: Vec<usize> = Vec::new();
            builder.build(&mut space);
            space.first_oop().become_free_oop();

            simple_garbage_collector::collect_from_roots(roots, &mut space);

            assert_eq!(space.first_oop().number_of_slots(), 239);
        }
    }
}
