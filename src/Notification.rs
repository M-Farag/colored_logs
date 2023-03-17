use colored::*;

pub struct Notification {
    messages: Vec<String>
}

impl Notification {
    pub fn new() -> Self
    {
        Self { messages: Vec::new() }
    }

    pub fn info(&self, msg:&String) -> ColoredString
    {
        msg.trim().yellow().bold()   
    }
    pub fn warning(&self, msg:&String) -> ColoredString
    {
        msg.trim().red().bold()   
    }

    pub fn store_message(&mut self,msg:String) {
        self.messages.push(msg);
    }

    pub fn messages(&self) -> &Vec<String>
    {
        &self.messages
    }


    
}
