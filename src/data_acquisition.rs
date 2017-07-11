
extern crate std;
use std::io::{stdin, BufRead, BufReader};
use state;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::fs::File;
use std::thread;
use std::time;

fn parse_line(line: &str, t: & state::RingDataBufferType) -> Option<state::RingData> {
    match *t {
        state::RingDataBufferType::Ints => {
            match line.parse::<i32>() {
                Ok(i) => Some(state::RingData::Int(i)),
                Err(msg) => {
                    println!("Expected an int, but got: {:?}", msg);
                    None
                }

            }
        },
        state::RingDataBufferType::Text => {None},
        state::RingDataBufferType::DatedInts => {None},
        state::RingDataBufferType::IntVec => {
            match line.split(' ').map(|v| v.parse::<i32>()).collect() {
                Ok(v) => Some(state::RingData::IntVec(v)),
                Err(msg) => {
                    println!("Error parsing int line. {:?}", msg);
                    None
                }
            }
        },
        _ => None
    }

}

pub fn stdin_reader(txdata: Sender<state::ChannelData>,
                    id: i32,
                    rdbtype: state::RingDataBufferType
                   ) {
    let sin = std::io::stdin();
    println!("starting stin_reader");
    for line in sin.lock().lines() {
        let line = line.unwrap();
        //println!("\nEntered: {:?}\n",line.clone());
        match line.clone().as_ref()  {
            "q" => ::std::process::exit(0),
            _ => {
                match parse_line(line.as_str(), & rdbtype) {
                    Some(parsed) => {
                        let cdat = state::ChannelData { id: id, dat: parsed };
                        txdata.send(cdat).unwrap();
                    }
                    None => {}
                }
            }
        }
    }
}

pub fn file_reader(txdata: Sender<state::ChannelData>,
                   id: i32,
                   filename: String,
                   rdbtype: state::RingDataBufferType
                  ) {
    let mut f = File::open(filename.clone()).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    println!("starting file_reader on {:?} for id {:?}", 
             filename.clone(), id.clone());
    loop {
        buffer = String::new();
        reader.read_line(&mut buffer);
        thread::sleep(time::Duration::from_millis(100));
        match parse_line(buffer.trim(), & rdbtype) {
            Some(parsed) => {
                //println!("file_reader got: {:?}", parsed.clone());
                let cdat = state::ChannelData { id: id, dat: parsed };
                txdata.send(cdat).unwrap();
            },
            None => {
                //println!("Expected an int, but got: {:?}", msg);
            }
        }
    }
}
