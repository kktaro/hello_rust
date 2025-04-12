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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_message_arguments() {
        let args = vec![
            String::from("arg1"),
            String::from("arg2"),
            String::from("arg3"),
        ];
        let message_arguments = MessageArguments::new(args);
        message_arguments.print_messages();
    }
}
