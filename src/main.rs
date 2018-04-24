use std::env;
use std::fs::File;
use std::io::Read;

mod bitwise;

struct Elem {
  count: u32,
  character: char,
}

struct CodeWord{
  len: usize,
  code: usize,
  character: char
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
    .map(|(a, b)| Elem {count: *b, character: a as char} )
    // .filter(|a| a.count != 0)
    .collect();

  alphabet.sort_by(|a, b| a.count.cmp(&b.count));
  let mut lengths : Vec<_> = alphabet.iter().map(|e| e.count as usize).collect();
  find_lengths(&mut lengths);
  let mut code = len_to_codewords(&lengths, &alphabet);
  code.sort_by(|a, b| a.character.cmp(&b.character));
  let s : Vec<_>= code.iter().map(|c| c.character as usize).collect();
  println!("{:?}", s);
}


#[test]
fn test_find_lengths() {
  let  data = &mut [1,1,1,1,5];
  find_lengths(data);
  assert_eq!(data, &mut [3,3,3,3,1]);

  let data = &mut [1,1,2,2,4];
  find_lengths(data);
  assert_eq!(data, &mut [3,3,2,2,2]); 
}

#[test]
fn hard_test_find_lengths() {
  let data = &mut [1, 1, 2, 3, 3, 4, 5, 5, 5, 5, 5, 6, 6];
  find_lengths(data);
  assert_eq!(data, &mut [6,6,5,4,4,4,4,4,3,3,3,3,3]);
}
fn find_lengths(data: &mut [usize]) {
  let mut zi = 0;
  let mut li = 1;
  let mut hi = 2;
  data[0] += data[1];

  while zi != data.len() - 2 {
    if hi < data.len() && (data[zi] >= data[hi] || data[zi + 1] >= data[hi] || zi == data.len() - 3) {
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
  println!("{:?}", data);
  println!("{:?}", li);
  for i in (0..li).rev() {
    data[i] = data[data[i]] + 1;
  }

  for i in (li + 1)..hi {
    data[i] = data.len();
  }

  let mut i : isize = data.len() as isize - 1;
  let mut depth = 1;
  let mut taken = 0;
  while i >= 0 {
    let mut available = 2isize.pow(depth as u32) - taken;
    available -= data.iter().filter(|&&e| e == depth).collect::<Vec<_>>().len() as isize;

    for j in 0..available {
      data[(i - j as isize) as usize] = depth;
    }

    i -= available;
    depth += 1;
    taken += available;
    taken *= 2;
  }
}

fn len_to_codewords(data: & [usize], elems: & [Elem]) -> Vec<CodeWord> {
  let mut count = 0;
  let mut last_index = 0;
  let mut result = Vec::new();
  for (&i, e) in data.iter().zip(elems).rev() {
    if i != last_index && last_index != 0 {
      count <<= i - last_index;
    }
    result.push(CodeWord{len: i, code: count, character: e.character});
    count += 1;
    last_index = i;
  }

  result.reverse();
  result
}