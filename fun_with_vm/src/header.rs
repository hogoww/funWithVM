pub struct Header{
	pub header_value: usize,
}

impl Header{
	// Multiple bits
	pub fn number_of_slots_bits(&self) -> usize {
		return self.header_value & 0xFF;
	}

	pub fn set_number_of_slots_bits(&self, number_of_slots: usize) -> usize {
		return (self.header_value & 0xFFFFFFFFFFFFFF00) | number_of_slots;
	}

	pub fn hash_bits(&self) -> usize {
		return (self.header_value & 0xFFFFFC00) >> 10;
	}

	pub fn set_hash_bits(&self, hash: usize) -> usize {
		return ((self.header_value & 0xFFFFFFFF000003FF) | hash) << 10;
	}


	pub fn format_bits(&self) -> usize {
		return (self.header_value & 0xFE00000000) >> 35;
	}


	pub fn set_format_bits(&self, format: usize) -> usize {
		return ((self.header_value & 0xFFFFFF01FFFFFFFF) | format) << 35;
	}


	pub fn class_index_bits(&self) -> usize {
		return (self.header_value & 0xFFFFFC0000000000) >> 42;
	}


	pub fn set_class_index_bits(&self, class_index: usize) -> usize {
		return ((self.header_value & 0x000003FFFFFFFFFF) | class_index) << 42;
	}

	// Individual Bits
	pub fn immutable_bit(&self) -> usize {
		return (self.header_value & 0x10000000000) >> 40;
	}


	pub fn set_immutable_bit(&self) -> usize {
		return self.header_value | 0x10000000000;
	}


	pub fn marked_bit(&self) -> usize {
		return (self.header_value & 0x1FF) >> 8;
	}


	pub fn set_marked_bit(&self) -> usize {
		return self.header_value | 0x100;
	}

	pub fn unset_marked_bit(&self) -> usize {
		return self.header_value & 0xFFFFFFFFFFFFFEFF;
	}


	pub fn pinned_bit(&self) -> usize {
		return (self.header_value & 0x200000000) >> 33;
	}

	pub fn set_pinned_bit(&self) -> usize {
		return self.header_value | 0x200000000;
	}


	pub fn grey_bit(&self) -> usize {
		return (self.header_value & 0x100000000) >> 32;
	}

	pub fn set_grey_bit(&self) -> usize {
		return self.header_value | 0x100000000;
	}

	pub fn remembered_bit(&self) -> usize {
		return (self.header_value & 0x10000000000) >> 34;
	}

	pub fn set_remembered_bit(&self) -> usize{
		return self.header_value | 0x10000000000;
	}
}
