use std::time::Instant;
use std::sync::LazyLock;

static INSTANT: LazyLock<Instant> = LazyLock::new(|| Instant::now());

fn print(subject: &str, message: &str) {
    let elapsed = INSTANT.elapsed();
    let secs = elapsed.as_secs();
    println!("{:6}:{:5}:{}", secs, subject, message)
}

pub fn info(message: &str) {
    print("INFO", message)
}

pub fn warn(message: &str) {
    print("WARN", message)
}

pub fn error(error: impl std::fmt::Display) {
    print("ERROR", &error.to_string())
}
