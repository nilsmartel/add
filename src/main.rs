use std::io::{stdin, Read, Result};

fn main() {
    let sum = stdin_to_string()
        .expect("failed to read from stdin")
        .split('\n')
        .map(|s| s.parse::<f64>())
        .fold(0.0, |a, r| {
            a + match r {
                Ok(n) => n,
                _ => 0.0,
            }
        });
    println!("{}", sum);
}
///
/// Reads the stdin to a String
/// Example
/// ```
/// let stdin = stdin_to_string().expect("failed to read stdin");
/// ```
fn stdin_to_string() -> Result<String> {
    let mut buffer = String::new();
    let stdin = stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}
