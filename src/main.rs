extern crate getopts;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::collections::BTreeMap;
use std::process::exit;

use getopts::Options;

mod bitwise;

struct Elem {
  count: usize,
  character: usize,
}

struct CodeWord{
  len: usize,
  code: usize,
  character: usize
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("c", "compress", "save compressed input file at output file");
  opts.optflag("d", "decompress", "save decompressed input file at output file");
  opts.optflag("o", "dictionary", "write the code words");
  opts.optflag("h", "help", "display this stuff");
  let matches = match opts.parse(&args[1..]) {
      Ok(m) => { m }
      Err(_) => { 
        command_line_msg(&program, &opts);
        return;
      }
  };

  if matches.opt_present("h") {
    command_line_msg(&program, &opts);
  }

  if matches.opt_present("c") && matches.opt_present("d") {
    command_line_msg(&program, &opts);
  } else if !matches.opt_present("c") && !matches.opt_present("d") && !matches.opt_present("o"){
    command_line_msg(&program, &opts);
  }

  let input_path = args[2].clone();

  let input_file = match File::open(input_path) {
    Ok(f) => f,
    Err(_) => {
      println!("couldn't open input file");
      exit(1);
    }
  };

  let output_path = args[3].clone();

  let output_file = match File::create(output_path) {
    Ok(f) => f,
    Err(_) => {
      println!("couldn't create output file");
      exit(1);
    }
  };

  if matches.opt_present("c") {
    compress(input_file, output_file);
  } else if matches.opt_present("d") {
    decompress(input_file, output_file);
  } else if matches.opt_present("o") {
    dictionary(input_file, output_file);
  } else {
    println!("must specify compress or decompress");
  }
}

fn command_line_msg(program : &String, opts : &Options) {
  let brief = format!("usage: {} [ -c | -d | -o] <input_file> <output_file>", program);
  println!("{}", opts.usage(&brief));
  exit(1);
}

fn compress(input_file : File, mut output_file: File) {
  let mut counts: [usize; 256] = [0; 256];
  let mut input = Vec::new();

  for b in input_file.bytes() {
    let byte = b.unwrap();
    counts[byte as usize] += 1;
    input.push(byte as usize);
  }

  let mut alphabet: Vec<Elem> = (0..256).zip(counts.iter())
    .map(|(a, &b)| Elem {count: b, character: a} )
    .filter(|a| a.count != 0)
    .collect();

  alphabet.push(Elem {count: 1, character: 256});

  alphabet.sort_by(|a, b| b.count.cmp(&a.count));

  let mut lengths : Vec<_> = alphabet.iter().map(|e| e.count as usize).collect();
  find_lengths(&mut lengths);

  for (a, l) in alphabet.iter_mut().rev().zip(lengths) {
    a.count = l;
  }

  alphabet.sort_by(|a, b| a.character.cmp(&b.character));
  alphabet.sort_by(|a, b| b.count.cmp(&a.count));

  let code = len_to_codewords(&alphabet);
  for index in 0..257 {
    let c = match code.iter().find(|&word| word.character == index) {
      Some(c) => c.len,
      None => 0
    };
    output_file.write(&[c as u8]).unwrap();
  }

  let mut output = bitwise::Writer::new(Box::new(output_file));
  
  println!("Writing Compressed File to out.txt");

  for b in input {
    let c = code.iter().find(|&word| word.character == b).unwrap();
    output.write(c.code, c.len);
  }

  let end_char = code.iter().find(|&word| word.character == 256).unwrap();
  output.write(end_char.code, end_char.len);
  output.flush();
  
  println!("Compression Complete");
}

fn decompress(input_file: File, mut output_file: File) {
  println!("Openining Compressed file");
  let mut alphabet : Vec<Elem> = Vec::new();

  let mut bytes = Box::new(input_file).bytes();
  for i in 0..257 {
    let len = bytes.next().unwrap().unwrap();
    if len > 0 {
      alphabet.push(Elem{count: len as usize, character: i});
    }
  }

  alphabet.sort_by(|a, b| a.character.cmp(&b.character));
  alphabet.sort_by(|a, b| b.count.cmp(&a.count));

  let code = len_to_codewords(&alphabet);

  let mut reader = bitwise::Reader::new(bytes);
  let mut read_values : BTreeMap<usize, Vec<CodeWord>> = BTreeMap::new();
  for c in code.into_iter().rev() {
    if read_values.contains_key(&c.len) {
      let mut values = read_values.get_mut(&c.len).unwrap();
      match values.binary_search_by(|x| c.code.cmp(&x.code)) {
        Ok(_) => {},
        Err(pos) => values.insert(pos, c)
      };
    } else {
      let mut values = Vec::new();
      let len = c.len;
      values.push(c);
      read_values.insert(len, values);
    }
  }

  let mut res = Vec::new();
  let mut done = false;

  println!("Decompressing File");

  while !done {
    let mut last_len = 0;
    let mut x = 0;
    for (&len, values) in read_values.iter() {
      x = match reader.read(len - last_len) {
        Ok(v) => {
          (x << (len - last_len)) | v
        },
        Err(_) => {
          done = true;
          break
        }
      };

      match values.binary_search_by(|c| x.cmp(&c.code)) {
        Ok(c) => {
          let value = &values[c];
          if value.character == 256 {
            done = true;
          } else {
            res.push(values[c].character as u8);
          }

          break
        },
        Err(_) => {
          last_len = len;
        }
      };
    }
  }

  println!("Writing Decompressed File");

  output_file.write(&res).unwrap();
}

fn dictionary(input_file: File, mut output_file: File) {
  let mut counts: [usize; 256] = [0; 256];
  let mut input = Vec::new();

  for b in input_file.bytes() {
    let byte = b.unwrap();
    counts[byte as usize] += 1;
    input.push(byte as usize);
  }

  let mut alphabet: Vec<Elem> = (0..256).zip(counts.iter())
    .map(|(a, &b)| Elem {count: b, character: a} )
    .filter(|a| a.count != 0)
    .collect();

  alphabet.push(Elem {count: 1, character: 256});

  alphabet.sort_by(|a, b| b.count.cmp(&a.count));

  let mut lengths : Vec<_> = alphabet.iter().map(|e| e.count as usize).collect();
  find_lengths(&mut lengths);

  for (a, l) in alphabet.iter_mut().rev().zip(lengths) {
    a.count = l;
  }

  alphabet.sort_by(|a, b| a.character.cmp(&b.character));
  alphabet.sort_by(|a, b| b.count.cmp(&a.count));

  let code = len_to_codewords(&alphabet);
  for c in code {
    writeln!(output_file, "{:3}: {:0len$b}", c.character, c.code, len = c.len).unwrap();
  }
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
    let mut temp;
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
    for i in zi..li {
      if temp < data[i] {
        let h = temp;
        temp = data[i];
        data[i] = h; 
        if ris.len() > 0 {
          for &j in ris.iter() {
            data[j] = i;
          }
          ris = Vec::new();
        }
      }
    }

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

fn len_to_codewords(elems: & [Elem]) -> Vec<CodeWord> {
  let mut count = 0;
  let mut last_index = 0;
  let mut result = Vec::new();
  for e in elems.into_iter().rev() {
    if e.count != last_index && last_index != 0 {
      count <<= e.count - last_index;
    }

    result.push(CodeWord{len: e.count, code: count, character: e.character});
    count += 1;
    last_index = e.count;
  }

  result.reverse();
  result
}