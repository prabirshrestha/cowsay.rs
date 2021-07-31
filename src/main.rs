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
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

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
