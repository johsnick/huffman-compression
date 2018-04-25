use std::io::Write;
use std::io::Read;
use std::boxed::Box;

pub struct Writer {
  pub buffer: u8,
  index: i8,
  output: Box<Write>
}

impl Writer {
  pub fn new(f: Box<Write>) -> Writer {
    Writer {
      output: f,
      index: 8,
      buffer: 0
    }
  }

  pub fn write(&mut self, word: u8, len: u8) {
    if self.index - len as i8 >= 0 {
      update_buffer(&mut self.buffer, &mut self.index, word, len);
      if self.index == 0 {
        self.output.write(&[self.buffer]).unwrap();
        self.index = 8;
      }
    } else {
      let upper_len = 8 - self.index;
      let upper_word = word >> (upper_len);
      let l = self.index as u8;
      update_buffer(&mut self.buffer, &mut self.index, upper_word, l);
      self.output.write(&[self.buffer]).unwrap();
      self.index = 8;
      update_buffer(&mut self.buffer, &mut self.index, word, len - l);
    }
  }

  pub fn flush(&mut self) {
    let i = self.index as u8;
    self.write(0, i);
  }
}

pub struct Reader {
  pub buffer: u8,
  index: i8,
  input: Box<Read>
}

impl Reader{
  pub fn new(f: Box<Read>) -> Reader {
    Reader {
      input: f,
      index: 0,
      buffer: 0
    }
  }

  pub fn read(&mut self, len: u8) -> u8 {
    let mut result;
    if self.index - len as i8 >= 0 {
      let mask = gen_input_mask(self.index as u8);
      result = (self.buffer & mask) >> (self.index - len as i8);
      self.index -= len as i8;
    } else if self.index == 0 {
      self.input.read_exact(&mut[self.buffer]);
      result = self.buffer >> (8 - len);
      self.index = 8 - len as i8;
    } else {
      let low_len = len - self.index as u8;
      let mask = gen_input_mask(self.index as u8);
      result = (self.buffer & mask) << (len - self.index as u8);
      self.input.read_exact(&mut[self.buffer]);
      result |= (self.buffer & mask) >> (8 - low_len);
      self.index = 8 - low_len as i8;
    }

    result
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

fn update_buffer(buffer: &mut u8, index: &mut i8, input: u8, len: u8) {
  *buffer &= gen_buffer_mask(*index);
  let input = input & gen_input_mask(len);
  *index -= len as i8;
  let (t, _) = input.overflowing_shl(*index as u32);
  *buffer |= t;
}

#[test]
fn test_gen_input_mask() {
  assert_eq!(gen_input_mask(1), 1);
  assert_eq!(gen_input_mask(4), 15);
  assert_eq!(gen_input_mask(5), 31);
  assert_eq!(gen_input_mask(8), 255);
}

fn gen_input_mask(len: u8) -> u8 {
  let mut mask = 0;
  for i in 0..len {
    mask |= 1 << i;
  }

  mask
}

#[test]
fn test_gen_buffer_mask() {
  assert_eq!(gen_buffer_mask(4), 240);
}

fn gen_buffer_mask(index: i8) -> u8 {
  let mut mask = 0;
  for i in index..8 {
    mask |= 1 << i;
  }

  mask
}
