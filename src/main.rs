use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
  let s = env::args().last().unwrap();
  let f = File::open(s).unwrap();

  let mut counts: [u8; 255] = [0; 255];

  for b in f.bytes() {
    let byte = b.unwrap();
    counts[byte as usize] += 1;
  }

  for c in counts.iter() {
    println!("{}", c);
  }
}