use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Moo!")]
    /// What does the cow say?
    message: String,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

    println!("{}", message);
    println!("   \\   ^__^");
    println!("     \\  (oo)\\______");
    println!("        (__)\\      )\\/\\");
    println!("           ||----w |");
    println!("           ||     ||");
}
