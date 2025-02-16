use base64::Base64;

mod base64;
mod sha;

fn main() {
    let mut base64 = Base64::new();
    let res = base64.encode("Hello").unwrap();
    println!("main res = {}", res);
    let decoded = base64.decode(res.as_str());
    println!("{}", decoded.unwrap());
}

#[cfg(test)]
mod tests {
    use super::base64::Base64;

    #[test]
    fn test_base64_encode_decode() {
        let original = "TESTING";
        let mut base64 = Base64::new();

        let encoded = base64.encode(original).unwrap();
        let decoded = base64.decode(&encoded).unwrap();

        assert_eq!(
            original, decoded,
            "Original and decoded messages do not match"
        );
    }
}
