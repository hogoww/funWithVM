use crate::header::Header;
use crate::memory_space::MemorySpace;
use crate::oop_common::oop_constants;
use crate::oop_common::OopCommonState;

#[derive(Debug)]
pub struct OopHeaders {
    header: Header,
    extra_header: usize,
}

impl OopCommonState for OopHeaders {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
    fn get_extra_header(&self) -> usize {
        self.extra_header
    }
}

impl OopHeaders {
    // Constructor
    pub fn new(index: usize, space: &MemorySpace) -> Self {
        let header = Header {
            header_value: space[index + oop_constants::HEADER_INDEX],
        };
        let extra_header = if header.has_extra_slot_header() {
            space[index + oop_constants::EXTRA_HEADER_INDEX]
        } else {
            oop_constants::NO_EXTRA_HEADER_VALUE
        };
        Self {
            header,
            extra_header,
        }
    }
}