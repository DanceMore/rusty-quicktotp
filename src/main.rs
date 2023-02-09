use dirs;
use std::fs;
use totp_rs::{Algorithm, TOTP, Secret};

fn main() {
    let mut filename = dirs::home_dir().unwrap();
    filename.push(".ga");

    let secret_string = fs::read_to_string(filename)
        .expect("Failed to read secret from file");
    let secret_string = secret_string.trim_end();

    let desired_length = 20;
    let mut secret = Secret::Encoded(secret_string.to_string()).to_bytes().unwrap();
    secret.resize(desired_length, 0x00);

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret,
    ).unwrap();
    let token = totp.generate_current().unwrap();
    println!("{}", token);
}
