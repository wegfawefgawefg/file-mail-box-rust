// write a file of fixed size of zeros

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

fn main() {
    let size = u64::pow(1024, 2); // 1 MB
    let mut file = File::create("zeros.bin").unwrap();
    file.seek(SeekFrom::Start(size as u64)).unwrap();
    file.write(&[0]).unwrap();
}

//
