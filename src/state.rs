use std::collections::VecDeque;


pub enum RingVizType {
    Hist, Interval, Text
}

// need... a data structure to fill from the thread

pub enum RingDataBuffer {
    Ints(VecDeque<i32>),
    Text(VecDeque<char>),
    Dates(VecDeque<i32>)
}

pub enum RingDataBufferType { Ints, Text, Dates }

impl RingDataBuffer {
    pub fn new(t: RingDataBufferType) -> RingDataBuffer {
            match t {
                RingDataBufferType::Ints => RingDataBuffer::Ints(VecDeque::new()),
                RingDataBufferType::Text => RingDataBuffer::Text(VecDeque::new()),
                RingDataBufferType::Dates => RingDataBuffer::Dates(VecDeque::new())
            }
    }
}

pub enum Actions {
    esc,
    enter,
    up,
    down,
    plus
}


pub struct WorldState {
    pub ioq: VecDeque<Actions>,
    pub data: Arc<Vec<RingDataBuffer>>
}

