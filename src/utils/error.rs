pub fn print_error(message: &str) {
    let result_message = String::from(message);
    eprintln!("{}", result_message);
}

pub fn print_error_with_error_message<T: std::fmt::Display>(message: &str, error: &T) {
    eprintln!("{}: {}", message, error);
}

pub fn print_error_and_exit(message: &str) -> ! {
    print_error(message);
    std::process::exit(1);
}

pub fn print_error_and_exit_with_error<T: std::fmt::Display>(message: &str, error: &T) -> ! {
    print_error_with_error_message(message, error);
    std::process::exit(1);
}
