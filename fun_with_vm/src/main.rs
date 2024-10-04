pub mod memory_space;
use crate::memory_space::MemorySpace;

fn main() {
	let memory_space_size:usize = 1000000;
	let space = MemorySpace::for_bit_size(memory_space_size);
	space.report();
}
