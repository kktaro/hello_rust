pub struct MessageArguments {
    messages: Vec<String>,
}

impl MessageArguments {
    pub fn new(args: Vec<String>) -> MessageArguments {
        MessageArguments { messages: args }
    }

    pub fn print_messages(&self) {
        let display_messages = self.messages.join(",");
        println!("Messages: {}", display_messages);
    }
}
