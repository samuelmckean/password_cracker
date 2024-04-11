use md5;

fn main() {
    let digest = md5::compute(b"hello world");
    println!("{}", format!("{:x}", digest));
}
