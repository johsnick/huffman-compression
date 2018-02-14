use std::env;
use std::fs::File;
use std::io::Read;

struct Elem {
  count: u32,
  character: char,
}

fn main() {
  let s =  match env::args().nth(1) {
    Some(s) => s,
    None => {
      println!("Error: must provide a filename");
      return
    }
  };

  let f = match File::open(s) {
    Ok(f) => f,
    Err(_) => {
      println!("Error: file not found");
      return
    }
  };

  let mut counts: [u32; 256] = [0; 256];

  for b in f.bytes() {
    let byte = b.unwrap();
    counts[byte as usize] += 1;
  }

  let mut map: Vec<_> = (0u8..255).zip(counts.iter())
    .filter(|&(_, c)| *c != 0)
    .map(|(a, b)| Elem {count: *b, character: a as char} )
    .collect();

  map.sort_by(|a,b| b.count.cmp(&a.count));

  for Elem {count, character} in map {
    println!("{}: {}", character as char, count);
  }
}