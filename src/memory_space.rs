use crate::memory_space_access::memory_space_access;
use crate::memory_space_access::MemorySpaceIterator;
use crate::oop_builder::OopBuilder;
use crate::oop_common::oop_utilities::how_many_headers_for;
use crate::oop_slice::OopSlice;
use crate::special_class_index::SpecialClassIndexes;

#[derive(Debug)]
pub struct MemorySpace {
    memory_vector: Vec<usize>,
}

impl MemorySpace {
    pub fn for_bit_size(memory_space_size: usize) -> Self {
        let mut res: Self = Self {
            memory_vector: vec![0; memory_space_size],
        };

        // set first oop to be free & have all the slots in the space
        let mut builder = OopBuilder::new();
        builder.set_class_index(SpecialClassIndexes::FreeObject as usize);
        builder.set_number_of_slots(memory_space_size - how_many_headers_for(memory_space_size));
        builder.build_oop_at(0, &mut res);

        res
    }

    pub fn get_start_index(&self) -> usize {
        0
    }

    pub fn get_end_index(&self) -> usize {
        self.memory_vector.capacity() - 1 // 0 based
    }

    pub fn first_oop(&mut self) -> OopSlice {
        memory_space_access::first_oop(self)
    }

    pub fn get_oop_at(&mut self, index: usize) -> OopSlice {
        memory_space_access::oop_at_index(index, self)
    }

    pub fn iter(&self) -> MemorySpaceIterator {
        MemorySpaceIterator::default()
    }

    pub fn report(&self) {
        println!("memory_vector = {}", self.memory_vector.len());
    }
}

impl<Idx> std::ops::Index<Idx> for MemorySpace
where
    Idx: std::slice::SliceIndex<[usize]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.memory_vector[index]
    }
}

impl<Idx> std::ops::IndexMut<Idx> for MemorySpace
where
    Idx: std::slice::SliceIndex<[usize]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.memory_vector[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::header::Header;
    use crate::memory_space::MemorySpace;
    use crate::oop_common::OopCommonState;
    use crate::oop_common::OopNavigation;

    #[test]
    fn test_unfilled_space_first_oop_is_free() {
        let mut space = MemorySpace::for_bit_size(240);
        assert!(space.first_oop().is_free_oop());
    }

    #[test]
    fn test_unfilled_space_first_oop_is_the_only_oop_in_space() {
        // The next index will be right after the end of the space
        let mut space = MemorySpace::for_bit_size(240);
        assert_eq!(
            space.first_oop().next_oop_index() - 1,
            space.get_end_index()
        );
    }

    #[test]
    fn test_bigger_space_allocate() {
        // The next index will be right after the end of the space
        let mut space = MemorySpace::for_bit_size(1000);
        assert_eq!(
            space.first_oop().next_oop_index() - 1,
            space.get_end_index()
        );
    }

    //In the following test cases, we test when the global free oop needs to go to use the extra header
    // Unfortunately, at the moment, it's a bit clumsy and we simply loose one slot.
    #[test]
    fn test_allocate_lower_bound_edge_case() {
        let mut space = MemorySpace::for_bit_size(Header::MAX_NUMBER_OF_SLOTS - 1);
        assert_eq!(
            space.first_oop().oop_size(),
            Header::MAX_NUMBER_OF_SLOTS - 1
        );
    }

    #[test]
    fn test_allocate_exact_bound_edge_case() {
        let mut space = MemorySpace::for_bit_size(Header::MAX_NUMBER_OF_SLOTS);
        assert_eq!(
            space.first_oop().oop_size(),
            Header::MAX_NUMBER_OF_SLOTS - 1
        );
    }

    #[test]
    fn test_allocate_middle_bound_edge_case() {
        let mut space = MemorySpace::for_bit_size(Header::MAX_NUMBER_OF_SLOTS + 1);
        assert_eq!(space.first_oop().oop_size(), Header::MAX_NUMBER_OF_SLOTS);
    }

    #[test]
    fn test_allocate_lower_bound_edge_case_end_index() {
        let space = MemorySpace::for_bit_size(Header::MAX_NUMBER_OF_SLOTS - 1);
        assert_eq!(space.get_end_index(), Header::MAX_NUMBER_OF_SLOTS - 2);
    }

    #[test]
    fn test_allocate_exact_bound_edge_case_end_index() {
        let space = MemorySpace::for_bit_size(Header::MAX_NUMBER_OF_SLOTS);
        assert_eq!(space.get_end_index(), Header::MAX_NUMBER_OF_SLOTS - 1);
    }

    #[test]
    fn test_allocate_middle_bound_edge_case_end_index() {
        let space = MemorySpace::for_bit_size(Header::MAX_NUMBER_OF_SLOTS + 1);
        assert_eq!(space.get_end_index(), Header::MAX_NUMBER_OF_SLOTS);
    }
}
