#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat{
    id: u64,
}

#[derive(Debug)]
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

struct GroundStation;

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

    let base = GroundStation{};

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
}
