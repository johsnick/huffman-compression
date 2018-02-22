use std::fs;
use std::io::Write;
use std::boxed::Box;
use std::rc;

pub struct Writer {
  pub buffer: u8,
  index: u8,
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
    if self.index - len >= 0 {
      self.update_buffer(self.buffer, self.index, word, len);
      if self.index == 0 {
        self.output.write(&[self.buffer]);
        self.index = 8;
      }
    } else {
      let upper_len = 8 - self.index;
      let upper_word = word >> (upper_len);
      update_buffer(self.buffer, self.index, upper_word, upper_len);
      self.output.write(&[self.buffer]);
      self.index = 8;
      update_buffer(self.buffer, self.index, word, len - upper_len);
    }
  }
}

#[test]
fn test_write() {
  let mut w = Writer::new(Box::new(Vec::new()));
  w.write(2,2);
  // w.write(2,6);
  assert_eq!(w.buffer, 130);
}

#[test]
fn test_update_buffer() {
  assert_eq!(update_buffer(192, 6, 3, 2), 240);
}

fn update_buffer(mut buffer: u8, mut index: u8, input: u8, len: u8) -> u8 {
  buffer &= gen_buffer_mask(index);
  let input = input & gen_input_mask(len);
  index -= len;
  buffer |= (input << index);
  buffer
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

fn gen_buffer_mask(index: u8) -> u8 {
  let mut mask = 0;
  for i in index..8 {
    mask |= 1 << i;
  }

  mask
}
