use std::io::Write;
use std::boxed::Box;

pub struct Writer {
  pub buffer: u8,
  index: i8,
  output: Box<Write>
}

impl Writer {
  pub fn new(f: Box<Write>) -> Writer {
    Writer {
      output: Box::new(f),
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
      update_buffer(&mut self.buffer, &mut self.index, upper_word, upper_len as u8);
      self.output.write(&[self.buffer]).unwrap();
      self.index = 8;
      update_buffer(&mut self.buffer, &mut self.index, word, len - upper_len as u8);
    }
  }
}

#[test]
fn test_write() {
  let mut w = Writer::new(Box::new(Vec::new()));
  w.write(1,2);
  assert_eq!(w.buffer, 64);
  assert_eq!(w.index, 6);
  w.write(1, 7);
  assert_eq!(w.index, 3);
  assert_eq!(w.buffer, 8);
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
  *buffer |= input << *index;
}

#[test]
fn test_gen_input_mask() {
  assert_eq!(gen_input_mask(1), 1);
  assert_eq!(gen_input_mask(4), 15);
  assert_eq!(gen_input_mask(5), 31);
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
