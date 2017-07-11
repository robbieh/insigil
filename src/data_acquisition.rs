
extern crate std;
use std::io::{stdin, BufRead, BufReader};
use state;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::fs::File;
use std::thread;
use std::time;

fn read_line(line: String) -> state::RingData {
    state::RingData::Int(5)
}

pub fn stdin_reader(txdata: Sender<state::ChannelData>,
    id: i32
    ) {
    let sin = std::io::stdin();
    println!("starting stin_reader");
    for line in sin.lock().lines() {
            let line = line.unwrap();
            //println!("\nEntered: {:?}\n",line.clone());
            match line.clone().as_ref()  {
                "q" => ::std::process::exit(0),
                _ => {}
            }
            match line.parse::<i32>() {
                Ok(i) => {
                    let rdint = state::RingData::Int(i);
                    let cdat = state::ChannelData { id: id, dat: rdint };
                    txdata.send(cdat).unwrap();
                },
                Err(msg) => {
                    println!("Expected an int, but got: {:?}", msg);
                }
            }
    }
}

pub fn file_reader(txdata: Sender<state::ChannelData>,
    id: i32,
    filename: String
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
        match buffer.trim().parse::<i32>() {
            Ok(i) => {
                //println!("got: {:?}", i.clone());
                let rdint = state::RingData::Int(i);
                let cdat = state::ChannelData { id: id, dat: rdint };
                txdata.send(cdat).unwrap();
            },
            Err(msg) => {
                //println!("Expected an int, but got: {:?}", msg);
            }
        }
    }
}
