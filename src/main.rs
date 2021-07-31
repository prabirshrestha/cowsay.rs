use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
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
