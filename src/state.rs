use std::collections::VecDeque;

use std::sync::{Arc, Mutex};

//#[derive(Debug)]
//pub enum RingVizType {
//    Hist, Interval, Text
//}

// need... a data structure to fill from the thread

#[derive(Debug,Clone)]
pub enum RingData {
    Int(i32),
    Text(String),
    Date((i32,i32)),
    IntVec(Vec<i32>)
}

#[derive(Debug,Clone)]
pub struct ChannelData {
  pub id: i32,
  pub dat: RingData
}

#[derive(Debug)]
pub enum RingDataBuffer {
    Ints(VecDeque<i32>),
    Text(VecDeque<char>),
    DatedInts(VecDeque<(i32,i32)>),
    IntVec(VecDeque<Vec<i32>>)
}

#[derive(Debug,Clone)]
pub enum RingDataBufferType { Ints, Text, DatedInts,IntVec }

impl RingDataBuffer {
    pub fn new(t: RingDataBufferType) -> RingDataBuffer {
        match t {
            RingDataBufferType::Ints => 
                RingDataBuffer::Ints(VecDeque::new()),
            RingDataBufferType::Text => 
                RingDataBuffer::Text(VecDeque::new()),
            RingDataBufferType::DatedInts => 
                RingDataBuffer::DatedInts(VecDeque::new()),
            RingDataBufferType::IntVec => 
                RingDataBuffer::IntVec(VecDeque::new())
        }
    }
}

#[derive(Debug)]
pub enum Actions {
    esc,
    enter,
    up,
    down,
    plus
}


