use std;
use common::*;
use message;

pub struct SegmentReader<'self> {
    messageReader : &'self message::MessageReader<'self>,
    segment : &'self [u8]
}


pub struct SegmentBuilder {
    segment : ~[u8],
    pos : WordCount
}

impl SegmentBuilder {

    pub fn new(size : ByteCount) -> SegmentBuilder {
        SegmentBuilder {
            segment : std::vec::from_elem(size, 0),
            pos : 0
        }
    }


    pub fn allocate(&mut self, amount : WordCount) -> Option<WordCount> {
        if (amount > self.segment.len() - self.pos) {
            return None;
        } else {
            let result = self.pos;
            self.pos += amount;
            return Some(result);
        }
    }
}

