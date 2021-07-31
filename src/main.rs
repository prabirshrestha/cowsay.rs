use std::io::Read;

use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Moo!")]
    /// What does the cow say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cow appear dead
    dead: bool,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() {
    let options = Options::from_args();

    let mut message = String::new();
    if options.stdin {
        std::io::stdin().read_to_string(&mut message).unwrap();
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cow shouldn't bark like a dog.");
        return;
    }

    let eye = if options.dead { "x" } else { "o" };

    println!("{}", message.bright_yellow().underline().on_purple());
    println!("   \\   ^__^");
    println!("     \\  ({}{})\\______", eye, eye);
    println!("        (__)\\      )\\/\\");
    println!("           ||----w |");
    println!("           ||     ||");
}
