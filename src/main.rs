use project_generator_cli::interact;

fn main() {
    match interact() {
        Ok(_) => println!("Project generated successfully"),
        Err(e) => eprintln!("Error generating project: {}", e),
    }
}
