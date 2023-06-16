use boringauth::oath::{HashFunction, TOTPBuilder};

fn main() {
    let key = "234567ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    #[allow(deprecated)]
    let totp = TOTPBuilder::new()
        .base32_key(key)
        .output_len(6)
        .hash_function(HashFunction::Sha1)
        .finalize()
        .unwrap();

    let token = totp.generate();

    println!("{token}");
}
