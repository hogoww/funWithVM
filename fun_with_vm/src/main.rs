pub mod memory_space;
use crate::memory_space::MemorySpace;
pub mod header;
use crate::header::Header;

fn main() {
	let memory_space_size:usize = 1000000;
	let space = MemorySpace::for_bit_size(memory_space_size);
	space.report();
	let header = Header {header_value: std::usize::MAX };
	println!("{}", header.number_of_slots_bits());
}
