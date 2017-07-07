
extern crate std;
use std::io::{stdin, BufRead};
use state;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub fn io_reader(txdata: Sender<state::ChannelData>,
    //textq: Arc<Mutex<VecDeque<String>>>
    id: i32
    ) {
    let sin = std::io::stdin();
    for line in sin.lock().lines() {
            let line = line.unwrap();
            println!("\nEntered: {:?}\n",line.clone());
            match line.clone().as_ref()  {
                "q" => ::std::process::exit(0),
                _ => {}
            }
            let in_int = match line.parse::<i32>() {
                Ok(i) => i,
                Err(msg) => {
                    println!("Expected an int, but got: {:?}", msg);
                    0
                }
            };
            //textq.lock().unwrap().push_back(line);
            let rdint = state::RingData::Int(in_int);
            let cdat = state::ChannelData { id: id, dat: rdint };
            txdata.send(cdat).unwrap();
    }
    //let msg = textq.lock().unwrap().pop_front();
    //if msg.is_some() {
    //    let msgstr: String = msg.unwrap();
   //     println!("I GOT MSG {:?}", msgstr.clone());
        //let mut iter = msgstr.split(' ');
    //    let v: Vec<&str> = msgstr.split(' ').collect();
        //iter.map(|num| println!("N: {:?}",num ));
     //   for i in v {
      //      println!("i: {:?}",i.clone()); 
            //rdbvec.get(0). 
    //    }

}
