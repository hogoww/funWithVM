pub mod allocator;
pub mod garbage_collector;
pub mod header;
pub mod memory_space;
pub mod memory_space_access;
pub mod oop_builder;
pub mod oop_common;
pub mod oop_headers;
pub mod oop_slice;
pub mod slot_content;
pub mod special_class_index;

use crate::header::Header;
use crate::memory_space::MemorySpace;
//use crate::oop::Oop;

use crate::oop_builder::OopBuilder;

//pub mod header_format_values;

fn main() {
    let memory_space_size: usize = 240;
    let mut space = MemorySpace::for_bit_size(memory_space_size);
    space.report();

    // let mut oop = Oop { memory: space, index: 0 };
    // println!("first oop number of slots, should have all slots in space(240): {}", oop.header_value());
    // println!("first oop class index, should be free (= 1): {}", oop.get_header() .class_index_bits());

    let header = Header {
        header_value: usize::MAX,
    };
    println!(
        "number of slots in full header: {} (should be 255)",
        header.number_of_slots_bits()
    );
    //oop.set_header(header);
    //println!("number of slots of oop with full header (should be 255): {}", oop.get_header().number_of_slots_bits());

    //getting ownership back
    //let mut space = oop.memory;
    let mut builder = OopBuilder::new();
    builder.set_class_index(3);
    builder.build(&mut space);

    // let oop = Oop { memory: space, index: 0 };
    // println!("class index of builder allocated oop (should be 3): {}", oop.get_header().class_index_bits());
}
