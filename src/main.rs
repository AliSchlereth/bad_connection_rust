use std::io;

fn main() {
    println!("HELLO, THIS IS A GROCERY STORE!");

    let mut listening = true;

    listen(listening);
}


fn listen(mut listening: bool) {
    while listening {
        let mut message = String::new();

        io::stdin().read_line(&mut message)
        .expect("Failed to read line");

        let clean_message = message.trim();

        listening = analyze_response(clean_message, listening);
    }
}

fn analyze_response(clean_message: &str, mut listening: bool) -> bool {
    let response = if clean_message == "" {
        "HELLO?!"
    } else if clean_message == "GOODBYE!" {
        listening = false;
        "THANK YOU FOR CALLING!"
    } else if clean_message == clean_message.to_uppercase() {
        "NO, THIS IS NOT A PET STORE"
    } else {
        "I AM HAVING A HARD TIME HEARING YOU."
    };

    print_response(response);
    listening
}

fn print_response(response: &str) {
    println!("{}", response);
}
