use std::env;

mod message_arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");

    let message_arguments = message_arguments::MessageArguments::new(args);
    message_arguments.print_messages();
}
