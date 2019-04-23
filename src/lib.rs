use std::path::Path;
use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug)]
pub struct LIBSVMParser<'libsvm> {
  pub file: &'libsvm Path
}

macro_rules! parse_file {
  ($type: ty, $file: expr) => {{
    let mut buf_reader = BufReader::new($file);
    let mut line = String::new();
    let mut data: Vec<Vec<$type>> = Vec::new();
    while buf_reader.read_line(&mut line)
      .expect("Error in reading file") > 0 {
      line = line.trim().to_string();
      let mut values = line.split_whitespace();
      let mut sample: Vec<$type> = Vec::new();
      let target = match values.next() {
        Some(target) => {
          match target.parse::<$type>() {
            Ok(target) => target,
            Err(_) => panic!("Error in parsing file.")
          }
        },
        None => panic!("Error in parsing file.")
      };
      sample.push(target);
      for value in values {
        let feature: Vec<&str> = value.split(':').collect();
        let feature = *(feature.last().unwrap());
        match feature.parse::<$type>() {
          Ok(f) => sample.push(f),
          Err(_) => panic!("Error in parsing file.")
        }
      }
      data.push(sample);
      line.clear();
    }
    data
  }};
}

impl<'libsvm> LIBSVMParser<'libsvm> {
  pub fn new(file: &'libsvm Path) -> Self {
    if file.exists() {
      let ext = match file.extension() {
        Some(ext) => ext,
        None => panic!("Wrong file format, expected .libsvm")
      };
      if ext != "libsvm" {
        panic!("Wrong file format, expected .libsvm but found: {:?}", ext);
      }
      Self {
        file: file
      }
    } else {
      panic!("File {:?} doesn't exist.", file);
    }
  }

  /// Read and parse libsvm file line by line.
  pub fn read_file<T: FromStr + Debug>(&self) -> Vec<Vec<T>> {
    let file = match File::open(self.file) {
      Ok(file) => file,
      Err(err) => panic!("Error in opening file. {:?}", err)
    };
    parse_file!(T, file)
  }
}