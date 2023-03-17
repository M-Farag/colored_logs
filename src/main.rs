use crate::Notification::Notification as Notify;
use std::io;

pub mod Notification;
fn main() {
    let mut n1 = Notify::new();
    'main_app_loop:loop {
        let mut user_input:String = String::new();
        println!("Write your message");
        io::stdin().read_line(&mut user_input).expect("Err reading STD in");
    
        println!("Info: {}",n1.info(&user_input));
        println!("Warning: {}",n1.warning(&user_input));

        n1.store_message(user_input);
        
        for msg in n1.messages() {
            print!("MSG: {}",msg);
        }
        print!("\n");
    }
    
}
