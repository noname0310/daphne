use std::{io, fs::File, time::Instant};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

fn main() {
    let start = Instant::now();
    for entry in WalkDir::new("D:/noname") {
        let entry = entry.unwrap();
        let mut file = File::open(entry.path().as_os_str()).unwrap();
        let mut sha256 = Sha256::new();
        io::copy(&mut file, &mut sha256).unwrap();
        let hash: [u8; 32] = sha256.finalize().into();
        println!("hash is: {:?}", hash);
        println!("{}", entry.path().display());
    }
    let duration = start.elapsed();
    println!("end of search take {:?}", duration);
}
