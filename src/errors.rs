use colored::Colorized;

fn error(msg: &str) -> ! {
    eprintln!(
        "{}",
        format!("Error: {}", msg).red()
    );
    process::exit(1);
}

fn warn(msg: &str) {
    eprintln!(
        "{}",
        format!("Warning: {}", msg).yellow()
    );
}