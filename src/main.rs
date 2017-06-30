use std::io;

fn main() {

    println!("HELLO, THIS IS A GROCERY STORE!");

    // let mut message = String::new();

    let mut listening = true;
    // let result = message == "\n";
    // println!("{}", result);


    while listening {
        let mut message = String::new();

        io::stdin().read_line(&mut message)
        .expect("Failed to read line");

        let clean_message = message.trim();

        if clean_message == "" {
            println!("HELLO?!");
        } else if clean_message == "GOODBYE!" {
            println!("THANK YOU FOR CALLING!");
            listening = false;
        } else if clean_message == clean_message.to_uppercase() {
            println!("NO, THIS IS NOT A PET STORE");
        } else {
            println!("I AM HAVING A HARD TIME HEARING YOU.");
        }
    }
}
