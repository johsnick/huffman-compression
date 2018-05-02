use std::io::Write;
use std::io::Bytes;
use std::boxed::Box;
use std::mem::size_of;
use std::mem::transmute;
use std::fs::File;

pub struct Writer {
  pub buffer: usize,
  index: isize,
  output: Box<Write>
}

impl Writer {
  pub fn new(f: Box<Write>) -> Writer {
    Writer {
      output: f,
      index: (size_of::<usize>() * 8)as isize,
      buffer: 0
    }
  }

  pub fn write(&mut self, word: usize, len: usize) {
    if self.index - len as isize > 0 {
      self.update_buffer(word, len);
    } else {
      let low_len = len - self.index as usize;
      let upper_word = word >> low_len;
      let l = self.index as usize;
      self.update_buffer(upper_word, l);
      self.write_to_file();
      self.update_buffer(word, low_len);
    }
  }

  pub fn flush(&mut self) {
    let temp : [u8; size_of::<usize>()] = unsafe { transmute(self.buffer.to_be()) };
    let bytes_to_write = ((size_of::<usize>() * 8) - self.index as usize) / 8;
    if ((size_of::<usize>() * 8) - self.index as usize) % 8 == 0 {
      self.output.write(&temp[0..bytes_to_write]).unwrap();
    } else {
      self.output.write(&temp[0..(bytes_to_write + 1)]).unwrap();
    }

    self.buffer = 0;
    self.index = (size_of::<usize>() * 8) as isize;
  }

  fn update_buffer(&mut self, input: usize, len: usize) {
    if len == 0 {
      return
    }

    self.index -= len as isize; 
    self.buffer |= (input & gen_input_mask(len)) << self.index;
  }

  fn write_to_file(&mut self) {
    let temp : [u8; size_of::<usize>()] = unsafe { transmute(self.buffer.to_be()) };
    self.output.write(&temp).unwrap();
    self.buffer = 0;
    self.index = (size_of::<usize>() * 8) as isize;
  }

}

pub struct Reader {
  pub buffer: usize,
  index: isize,
  input: Bytes<Box<File>>,
}

impl Reader{
  pub fn new(f: Bytes<Box<File>>) -> Reader {
    Reader {
      input: f,
      index: 0,
      buffer: 0,
    }
  }

  pub fn read(&mut self, len: usize) -> Result<usize, ()> {
    let mut result;
    if self.index - len as isize >= 0 {
      let mask = gen_input_mask(self.index as usize);
      result = (self.buffer & mask) >> (self.index - len as isize);
      self.index -= len as isize;
    } else {
      let low_len = len - self.index as usize;
      let mask = gen_input_mask(self.index as usize);
      result = (self.buffer & mask) << (len - self.index as usize);
      match self.read_byte() {
        Err(_) => return Err(()),
        Ok(_) => {}
      };
      result |= self.buffer >> (self.index - low_len as isize);
      self.index -= low_len as isize;
    }


    Ok(result)
  }

  fn read_byte(&mut self) -> Result<(), ()> {
    self.buffer = 0;
    for i in (0..size_of::<usize>()).rev() {
      match self.input.next() {
        Some(x) => {
          self.buffer |= (x.unwrap() as usize) << (i * 8)
        },
        None => {
          if i != size_of::<usize>() - 1 {
            self.buffer >>= (i + 1) * 8;
            self.index = ((size_of::<usize>() - i - 1) as usize) as isize * 8;
            return Ok(());
          } else {
            return Err(());
          }
        }
      }
    }

    self.index = 8 * size_of::<usize>() as isize;
    Ok(())
  }
}

#[test]
fn test_write() {
  let mut w = Writer::new(Box::new(Vec::new()));
  w.write(1,2);
  assert_eq!(w.buffer, 64);
  assert_eq!(w.index, 6);
  w.write(1, 7);
  assert_eq!(w.index, 7);
  assert_eq!(w.buffer, 128);
}

#[test]
fn test_update_buffer() {
  let mut index = 6;
  let mut buffer = 192;
  update_buffer(&mut buffer, &mut index, 3, 2);
  assert_eq!(buffer, 240);
  assert_eq!(index, 4);
}


#[test]
fn test_gen_input_mask() {
  assert_eq!(gen_input_mask(0), 0);
  assert_eq!(gen_input_mask(1), 1);
  assert_eq!(gen_input_mask(4), 15);
  assert_eq!(gen_input_mask(5), 31);
  assert_eq!(gen_input_mask(8), 255);
}

fn gen_input_mask(len: usize) -> usize {
  let mut mask = 0;
  for i in 0..len {
    mask |= 1 << i;
  }

  mask
}
