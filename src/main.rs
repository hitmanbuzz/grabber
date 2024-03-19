mod source;

use std::io::{self, stdout, Write};

use crate::source::readm::readm;

fn main() {
    if cli_options() == "1" {
        readm();
    }
    else {
        println!("Wrong option!!!");
    }
}

fn cli_options() -> String {
    print!("
[1] READM (readm.today)
[?] More coming soon

[#] Choose: ");
    stdout().flush().unwrap();
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("failed to take input for option");
    let option = option.trim();
    return option.to_string();
}