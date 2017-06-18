
extern crate std;
use std::io::{stdin, BufRead};
use state;

pub fn io_reader(world: &mut state::WorldState
    //textq: Arc<Mutex<VecDeque<String>>>
    ) {
    let sin = std::io::stdin();
    for line in sin.lock().lines() {
            let line = line.unwrap();
            println!("Entered: {:?}",line.clone());
            //textq.lock().unwrap().push_back(line);

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
