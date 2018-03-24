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

  let mut alphabet: Vec<Elem> = (0u8..255).zip(counts.iter())
    .filter(|&(_, c)| *c != 0)
    .map(|(a, b)| Elem {count: *b, character: a as char} )
    .collect();

  alphabet.sort_by(|a, b| a.count.cmp(&b.count));
  let mut vdata : Vec<_> = alphabet.iter().map(|e| e.count).collect::<Vec<_>>();
  let data = &mut vdata;
  println!("{:?}", data);
}


#[test]
fn test_find_lengths() {
  let  data = &mut [2,2,2,2,200];
  find_lengths(data);
  assert_eq!(data, &mut [3,3,2,2,1]);

  let data = &mut [1,1,2,2,4];
  find_lengths(data);
  assert_eq!(data, &mut [3,3,2,2,2]); 
}

fn find_lengths(data: &mut [usize]) {
  let mut zi = 0;
  let mut li = 1;
  let mut hi = 2;
  data[0] += data[1];

  while hi != data.len() {
    if data[zi] >= data[hi] || data[zi + 1] >= data[hi] || zi == data.len() - 3 {
      if data.len() != hi + 1 && data[zi] >= data[hi + 1] {
        data[li] = data[hi] + data[hi + 1];
        hi += 2;
        li += 1;
      } else {
        data[li] = data[zi] + data[hi];
        data[zi] = li;
        zi += 1;
        li += 1;
        hi += 1;
      }
    } else {
      data[li] = data[zi] + data[zi + 1];
      data[zi] = li;
      data[zi + 1] = li;
      li += 1;
      zi += 2;
    }
  }

  if data.len() - li == 2 {
    data[li] = data[zi] + data[zi + 1];
    data[zi] = li;
    data[zi + 1] = li;
    li += 1;
  }
  li -= 1;
  data[li] = 0;
  for i in (0..li).rev() {
    data[i] = data[data[i]] + 1;
  }

  for i in (li + 1)..hi {
    data[i] = data.len();
  }

  let mut i : isize = data.len() as isize - 1;
  let mut depth = 1;
  while i >= 0 {
    let total = depth << 1;
    let found = total as isize - data.iter().filter(|&&e| e == depth).collect::<Vec<_>>().len() as isize;
    for j in (0..found) {
      if j > i {
        break;
      }

      data[(i - j as isize) as usize] = depth;
    }

    i -= found;
    depth += 1;
  }
}