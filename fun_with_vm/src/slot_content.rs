//use crate::oop::Oop;

#[derive(Debug)]
pub struct SlotContent {
    content: usize,
}

impl SlotContent {
    // Constructor
    pub fn new(slot_content: usize) -> Self {
        Self {
            content: slot_content,
        }
    }

    // Accessing
    pub fn get_content(&self) -> usize {
        self.content
    }

    // Testing
    // todo(immediate)
    pub fn is_slot_immediate(&self) -> bool {
        return false;
    }

    // todo(immediate)
    pub fn is_slot_oop(&self) -> bool {
        return true;
    }

    // todo conversions;
    // pub fn as_oop(&self) -> Oop {
    // }

    // pub fn as_immediate(&self) -> Immediate {
    // }
}
