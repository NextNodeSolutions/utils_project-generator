use clap::Parser;
use project_generator_cli::interact;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Set debug mode in the global context
    project_generator_cli::utils::context::set_debug_mode(args.debug);

    match interact() {
        Ok(_) => println!("Project generated successfully"),
        Err(e) => eprintln!("Error generating project: {}", e),
    }
}
