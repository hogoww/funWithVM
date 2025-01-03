use crate::special_class_index::SpecialClassIndexes;

#[derive(Debug)]
pub struct Header {
    pub header_value: usize,
}

impl Header {
    pub fn new() -> Self {
        Self { header_value: 0 }
    }

    pub fn get_value(&self) -> usize {
        self.header_value
    }

    // Constants
    pub const MAX_NUMBER_OF_SLOTS: usize = 254;
    const EXTRA_SLOT_HEADER: usize = 0xFF;

    // Multiple bits
    pub fn number_of_slots_bits(&self) -> usize {
        self.header_value & 0xFF
    }

    pub fn set_number_of_slots_bits(&mut self, number_of_slots: usize) {
        if number_of_slots > Header::MAX_NUMBER_OF_SLOTS {
            panic!("Tried to set number of slots {} directly in the header. Headers only support {} slots", number_of_slots , Header::MAX_NUMBER_OF_SLOTS)
        }
        self.header_value = (self.header_value & 0xFFFFFFFFFFFFFF00) | number_of_slots;
    }

    pub fn set_number_of_slots_to_max(&mut self) {
        self.header_value = (self.header_value & 0xFFFFFFFFFFFFFF00) | Header::EXTRA_SLOT_HEADER;
    }

    pub fn hash_bits(&self) -> usize {
        (self.header_value & 0xFFFFFC00) >> 10
    }

    pub fn set_hash_bits(&mut self, hash: usize) {
        self.header_value = (self.header_value & 0xFFFFFFFF000003FF) | (hash << 10);
    }

    pub fn format_bits(&self) -> usize {
        (self.header_value & 0xFE00000000) >> 35
    }

    pub fn set_format_bits(&mut self, format: usize) {
        self.header_value = (self.header_value & 0xFFFFFF01FFFFFFFF) | (format << 35);
    }

    pub fn class_index_bits(&self) -> usize {
        (self.header_value & 0xFFFFFC0000000000) >> 42
    }

    pub fn set_class_index_bits(&mut self, class_index: usize) {
        self.header_value = (self.header_value & 0x000003FFFFFFFFFF) | (class_index << 42);
    }

    // Individual Bits
    pub fn immutable_bit(&self) -> usize {
        (self.header_value & 0x10000000000) >> 40
    }

    pub fn set_immutable_bit(&mut self) {
        self.header_value |= 0x10000000000;
    }

    pub fn marked_bit(&self) -> usize {
        (self.header_value & 0x1FF) >> 8
    }

    pub fn set_marked_bit(&mut self) {
        self.header_value |= 0x100;
    }

    pub fn unset_marked_bit(&mut self) {
        self.header_value &= 0xFFFFFFFFFFFFFEFF;
    }

    pub fn pinned_bit(&self) -> usize {
        (self.header_value & 0x200000000) >> 33
    }

    pub fn set_pinned_bit(&mut self) {
        self.header_value |= 0x200000000;
    }

    pub fn grey_bit(&self) -> usize {
        (self.header_value & 0x100000000) >> 32
    }

    pub fn set_grey_bit(&mut self) {
        self.header_value |= 0x100000000;
    }

    pub fn remembered_bit(&self) -> usize {
        (self.header_value & 0x10000000000) >> 40
    }

    pub fn set_remembered_bit(&mut self) {
        self.header_value |= 0x10000000000;
    }

    pub fn unset_remembered_bit(&mut self) {
        self.header_value &= 0xFFFFFEFFFFFFFFFF;
    }

    // testing
    pub fn has_extra_slot_header(&self) -> bool {
        self.number_of_slots_bits() == Header::EXTRA_SLOT_HEADER
    }

    pub fn is_free_oop(&self) -> bool {
        self.class_index_bits() == SpecialClassIndexes::FreeObject as usize
    }

    // reclaiming
    pub fn become_free_oop(&mut self) {
        self.set_class_index_bits(SpecialClassIndexes::FreeObject as usize);
    }

    pub fn header_size(&self) -> usize {
        if self.has_extra_slot_header() {
            1 + 1
        } else {
            1
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Header;

    #[test]
    fn test_class_index() {
        let header = Header {
            header_value: 0xFFFFFC0000000000,
        };
        assert_eq!(header.class_index_bits(), 0x3FFFFF);
    }

    #[test]
    fn test_format() {
        let header = Header {
            header_value: 0xF800000000,
        };
        assert_eq!(header.format_bits(), 0x1F);
    }

    #[test]
    fn test_grey_bit() {
        let header = Header {
            header_value: 0x100000000,
        };
        assert_eq!(header.grey_bit(), 1);
    }

    #[test]
    fn test_hash() {
        let header = Header {
            header_value: 0xFFFFFC00,
        };
        assert_eq!(header.hash_bits(), 0x3FFFFF);
    }

    #[test]
    fn test_immutable_bit() {
        let header = Header {
            header_value: 0x10000000000,
        };
        assert_eq!(header.immutable_bit(), 1);
    }

    #[test]
    fn test_marked_bit() {
        let header = Header {
            header_value: 0xF1FF,
        };
        assert_eq!(header.marked_bit(), 1);
    }

    #[test]
    fn test_not_class_index() {
        let header = Header {
            header_value: 0x3FFFFFFFFFF,
        };
        assert_eq!(header.class_index_bits(), 0);
    }

    #[test]
    fn test_not_format() {
        let header = Header {
            header_value: 0xFFFFFF07FFFFFFFF,
        };
        assert_eq!(header.format_bits(), 0);
    }

    #[test]
    fn test_not_grey_bit() {
        let header = Header {
            header_value: 0xFFFFFF8FFFFFFFF,
        };
        assert_eq!(header.grey_bit(), 0);
    }

    #[test]
    fn test_not_hash() {
        let header = Header {
            header_value: 0xFFFFFFFF000003FF,
        };
        assert_eq!(header.hash_bits(), 0);
    }

    #[test]
    fn test_not_immutable_bit() {
        let header = Header {
            header_value: 0xFFFFFEFFFFFFFFFF,
        };
        assert_eq!(header.immutable_bit(), 0);
    }

    #[test]
    fn test_not_marked_bit() {
        let header = Header {
            header_value: 0xFFFFFFFFFFFFFEFF,
        };
        assert_eq!(header.marked_bit(), 0);
    }

    #[test]
    fn test_not_number_of_slots() {
        let header = Header {
            header_value: 0xFFFFFFFFFFFFFF00,
        };
        assert_eq!(header.number_of_slots_bits(), 0);
    }

    #[test]
    fn test_not_pinned_bit() {
        let header = Header {
            header_value: 0xFFFFFFFDFFFFFFFF,
        };
        assert_eq!(header.pinned_bit(), 0);
    }

    #[test]
    fn test_not_remembered_bit() {
        let header = Header {
            header_value: 0xFFFFFEFFFFFFFFFF,
        };
        assert_eq!(header.remembered_bit(), 0);
    }

    #[test]
    fn test_number_of_slots() {
        let header = Header { header_value: 0xFF };
        assert_eq!(header.number_of_slots_bits(), 0xFF);
    }

    #[test]
    fn test_pinned_bit() {
        let header = Header {
            header_value: 0x200000000,
        };
        assert_eq!(header.pinned_bit(), 1);
    }

    #[test]
    fn test_remembered_bit() {
        let header = Header {
            header_value: 0x10000000000,
        };
        assert_eq!(header.remembered_bit(), 1);
    }

    #[test]
    fn test_set_class_index() {
        let class_index: usize = 4;
        let mut header = Header::new();
        header.set_class_index_bits(class_index);
        assert_eq!(header.class_index_bits(), class_index);
    }

    #[test]
    fn test_set_format() {
        let format: usize = 4;
        let mut header = Header::new();
        header.set_format_bits(format);
        assert_eq!(header.format_bits(), format);
    }

    #[test]
    fn test_set_grey_bit() {
        let mut header = Header::new();
        header.set_grey_bit();
        assert_eq!(header.grey_bit(), 1);
    }

    #[test]
    fn test_set_hash() {
        let hash: usize = 549;
        let mut header = Header::new();
        header.set_hash_bits(hash);
        assert_eq!(header.hash_bits(), hash);
    }

    #[test]
    fn test_set_immutable_bit() {
        let mut header = Header::new();
        header.set_immutable_bit();
        assert_eq!(header.immutable_bit(), 1);
    }

    #[test]
    fn test_set_marked_bit() {
        let mut header = Header::new();
        header.set_marked_bit();
        assert_eq!(header.marked_bit(), 1);
    }

    #[test]
    fn test_set_number_of_slots() {
        let number_of_slots: usize = 5;
        let mut header = Header::new();
        header.set_number_of_slots_bits(number_of_slots);
        assert_eq!(header.number_of_slots_bits(), number_of_slots);
    }

    #[test]
    fn test_set_pinned_bit() {
        let mut header = Header::new();
        header.set_pinned_bit();
        assert_eq!(header.pinned_bit(), 1);
    }

    #[test]
    fn test_set_remembered_bit() {
        let mut header = Header::new();
        header.set_remembered_bit();
        assert_eq!(header.remembered_bit(), 1);
    }

    #[test]
    fn test_set_remembered_bit_after_number_of_slots_keeps_slots() {
        let mut header = Header::new();
        header.set_number_of_slots_bits(42);
        header.set_remembered_bit();
        assert_eq!(header.number_of_slots_bits(), 42);
    }

    #[test]
    fn test_unset_remembered_bit_after_number_of_slots_keeps_slots() {
        let mut header = Header::new();
        header.set_number_of_slots_bits(42);
        header.unset_remembered_bit();
        assert_eq!(header.number_of_slots_bits(), 42);
    }

    #[test]
    fn test_set_marked_bit_after_number_of_slots_keeps_slots() {
        let mut header = Header::new();
        header.set_number_of_slots_bits(42);
        header.set_marked_bit();
        assert_eq!(header.number_of_slots_bits(), 42);
    }

    #[test]
    fn test_unset_marked_bit_after_number_of_slots_keeps_slots() {
        let mut header = Header::new();
        header.set_number_of_slots_bits(42);
        header.unset_marked_bit();
        assert_eq!(header.number_of_slots_bits(), 42);
    }

    #[test]
    fn test_set_class_index_after_number_of_slots_keeps_slots() {
        let mut header = Header::new();
        header.set_number_of_slots_bits(42);
        header.set_class_index_bits(3);
        assert_eq!(header.number_of_slots_bits(), 42);
    }

    #[test]
    fn test_set_format_after_number_of_slots_keeps_slots() {
        let mut header = Header::new();
        header.set_number_of_slots_bits(42);
        header.set_format_bits(3);
        assert_eq!(header.number_of_slots_bits(), 42);
    }

    #[test]
    fn test_set_hash_after_number_of_slots_keeps_slots() {
        let mut header = Header::new();
        header.set_number_of_slots_bits(42);
        header.set_hash_bits(3);
        assert_eq!(header.number_of_slots_bits(), 42);
    }

    #[test]
    fn test_set_number_of_slots_after_hash_keep_hash() {
        let mut header = Header::new();
        header.set_hash_bits(3);
        header.set_number_of_slots_bits(42);
        assert_eq!(header.number_of_slots_bits(), 42);
    }

    #[test]
    fn test_small_oop_does_not_have_extra_header() {
        let mut header = Header::new();
        header.set_number_of_slots_bits(42);
        assert!(!header.has_extra_slot_header());
    }

    #[test]
    fn test_big_oop_has_extra_header() {
        let mut header = Header::new();
        header.set_number_of_slots_to_max();
        assert!(header.has_extra_slot_header());
    }
}
