pub mod memory_space;
use crate::memory_space::MemorySpace;
pub mod header;
use crate::header::Header;

pub mod oop;
use crate::oop::Oop;

fn main() {
	let memory_space_size:usize = 1000000;
	let space = MemorySpace::for_bit_size(memory_space_size);
	space.report();
	let header = Header { header_value: std::usize::MAX };
	println!("number of slots in full header: {}", header.number_of_slots_bits());

	let mut oop = Oop { memory: space, index: 0};
	println!("number of slots with empty header: {}", oop.header_value());
	oop.set_header(header);
	println!("number of slots of oop  with full header: {}", oop.get_header().number_of_slots_bits());
}
