use std::collections::VecDeque;
use graphics::types::Color;

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

//#[derive(Debug)]
pub enum RingDataBuffer {
    Ints(VecDeque<i32>),
    Text(VecDeque<char>),
    DatedInts(VecDeque<(i32,i32)>),
    IntVec(VecDeque<Vec<i32>>)
}



#[derive(Debug)]
pub enum Actions {
    Esc,
    Enter,
    Up,
    Down,
    Plus
}

#[derive(Debug, Clone, Deserialize)]
pub struct Palette {
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub highlight: Color
}
