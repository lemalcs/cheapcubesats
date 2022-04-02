#![allow(unused_variables)]

use std::{rc::Rc, cell::RefCell};

// `Copy` trait creates an exact copy of a value
// `Clone` trait creates light copy of a value and may differ 
// from original value.
// Copy implies Clone.
#[derive(Debug, Clone, Copy)]
struct CubeSat{
    id: u64,
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage{
    Ok,
}

#[derive(Debug)]
struct Message{
    to: u64,
    content: String,
}

#[derive(Debug)]
struct MailBox{
    messages: Vec<Message>,
}

impl MailBox{
    // Mutable access to itself
    // Take ownership to Message
    fn post(&mut self, msg: Message){
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message>{
        for i in 0..self.messages.len(){
            if self.messages[i].to == recipient.id{

                // Caution: modify the collection while iterating is an anti-pattern
                // This time is legal because of the return expression when a message is found
                let msg = self.messages.remove(i);
                return Some(msg) // return the Message if found
            }
        }

        None
    }
}

#[derive(Debug)]
struct GroundStation{
    radio_freq: f64, // Mhz
}

impl GroundStation{
    // Read only access to `self`
    // Take a mutable borrow to `CubeSat`
    // Take full ownership of `msg`
    fn send(&self,mailbox: &mut MailBox, msg: Message){
        // Ownership of Message instace transferred to message.push()
        mailbox.post(msg);
    }

    fn connect(&self, sat_id:u64) -> CubeSat{
        CubeSat{
            id:sat_id,
        }
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut MailBox)-> Option<Message>{
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64>{
    vec![1,2,3]
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?} : {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {

    let mut mailbox= MailBox{messages: vec![]};

    let base = GroundStation{
        radio_freq : 84.32
    };

    let sat_ids= fetch_sat_ids();

    for sat_id in sat_ids{
        let sat = base.connect(sat_id);
        let msg = Message{to:sat.id, content:String::from("hello"),};
        base.send(&mut mailbox, msg);
    }

    let sat_ids= fetch_sat_ids();
    for sat_id in sat_ids{
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mailbox);
        println!("{:?} : {:?}", sat,msg);
    }

    let sat_a = CubeSat{ id: 0 };
    let a_status = check_status(sat_a.clone());
    println!("a: {:?}",a_status.clone());
    
    // Copy trait create a new value of variable `sat_a`
    let a_status = check_status(sat_a);
    println!("a: {:?}",a_status);

    create_wrapped_ground_stations();
}

fn create_wrapped_ground_stations(){

    // `Rc` type provides shared ownership
    // This variable dows not allow mutation
    let base = Rc::new(GroundStation
        {
            radio_freq : 84.33
        });
    println!("{:?}",base);

    // `RefCell` type allows mutation
    let base2:Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(GroundStation
        {
            radio_freq : 84.34
        }));
    println!("{:?}",base2);

    // Optional new scope added
    {
        // base2 is  mutably borrowed
        let mut base_3=base2.borrow_mut();
        base_3.radio_freq=93.7;
        println!("{:?}",base_3);
    }
}