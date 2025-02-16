use sha::Sha1;

mod sha;

fn main() {
    let mut sha1 = Sha1::new();
    let hash = sha1.hash("hello its me");
    let hash1 = sha1.hash("hello its me");
    let hash2 = sha1.hash("hello its me2");
    let hash3 = sha1.hash("hello its me3");
    println!("result hash is {:?}", hash);
    println!("result1 hash is {:?}", hash1);
    println!("result2 hash is {:?}", hash2);
    println!("result3 hash is {:?}", hash3);
}
