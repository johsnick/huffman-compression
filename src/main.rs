use std::env;
use std::fs::File;
use std::io::Read;

mod bitwise;

struct Elem {
  count: usize,
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

  let f = match File::open(&s) {
    Ok(f) => f,
    Err(_) => {
      println!("Error: file not found");
      return
    }
  };

  let mut counts: [usize; 256] = [0; 256];
  let mut input = Vec::new();

  for b in f.bytes() {
    let byte = b.unwrap();
    counts[byte as usize] += 1;
    input.push(byte);
  }

  let mut alphabet: Vec<Elem> = (0u8..255).zip(counts.iter())
    .map(|(a, &b)| Elem {count: b, character: a as char} )
    .filter(|a| a.count != 0)
    .collect();

  alphabet.sort_by(|a, b| a.count.cmp(&b.count));
  let mut lengths : Vec<_> = alphabet.iter().map(|e| e.count as usize).collect();
  find_lengths(&mut lengths);
  let code = len_to_codewords(&lengths, &alphabet);

  for ref c in code.iter() {
    println!("{:?}:\t{:4b} - {:?}", c.character, c.code, c.len);
  }

  let out_f = match File::create("out.txt") {
    Ok(f) => f,
    Err(_) => {
      println!("Error: file not found");
      return
    }
  };

  let mut output = bitwise::Writer::new(Box::new(out_f));
  
  for b in input {
    let c = code.iter().find(|&word| word.character == b as char).unwrap();
    output.write(c.code, c.len);
  }
  println!("");

  output.flush();

  let in_f = match File::open("out.txt") {
    Ok(f) => f,
    Err(_) => {
      println!("you have fucked up now");
      return
    }
  };

  let mut reader = bitwise::Reader::new(Box::new(in_f));
  let mut read_values = Vec::new();
  for &len in lengths.iter().rev() {
    match read_values.iter().find(|&&x| x == len) {
      None => read_values.push(len),
      Some(_) => {}
    };
  }

  let mut res = String::new();
  let mut done = false;
  while !done {
    let mut last_len = 0;
    let mut x = 0;
    for &len in read_values.iter() {
      x = match reader.read(len - last_len) {
        Ok(v) => {
          (x << (len - last_len)) | v
        },
        Err(_) => {
          done = true;
          break
        }
      };

      match code.iter().find(|&c| c.code == x as usize) {
        Some(c) => {
          res.push(c.character);
          break
        },
        None => {
          last_len = len;
        }
      };
    }
  }

  println!("{:?}", res);
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
  while li != data.len() - 1 { 
    let temp;
    let mut ris = Vec::new();
    if hi == data.len() || data[zi] < data[hi] && data[zi + 1] < data[hi] {
      temp = data[zi] + data[zi + 1];
      ris.push(zi);
      ris.push(zi + 1);
      zi += 2;
    } else if hi < data.len() - 1 && data[zi] >= data[hi + 1] {
      temp = data[hi] + data[hi + 1];
      hi += 2;
    } else {
      temp = data[zi] + data[hi];
      ris.push(zi);
      hi += 1;
      zi += 1;
    }

// this code might fix weird issues
// or be pointless 
    // for i in zi..li {
    //   if temp < data[i] {
    //     let h = temp;
    //     temp = data[i];
    //     data[i] = h; 
    //     if ris.len() > 0 {
    //       for &j in ris.iter() {
    //         data[j] = i;
    //       }
    //       ris = Vec::new();
    //     }
    //   }
    // }

    data[li] = temp;
    if ris.len() > 0 {
      for &j in ris.iter() {
        data[j] = li;
      }
    }

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

fn len_to_codewords(data: &[usize], elems: & [Elem]) -> Vec<CodeWord> {
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