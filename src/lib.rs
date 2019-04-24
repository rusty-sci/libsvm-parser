/// LIBSVMParser is for parsing libsvm file format.
use std::path::Path;
use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct LIBSVMParser {
  is_classif: bool
}

impl LIBSVMParser {

  pub fn new() -> Self {
    Self {
      is_classif: false
    }
  }

  /// If init with true, than read_file returns HashSet with unique classes.
  pub fn is_classification(self, is_classification: bool) -> Self {
    Self {
      is_classif: is_classification,
      ..self
    }
  }

  /// Read and parse libsvm file line by line.
  pub fn parse_file<T, U>(&self, file: &Path) ->
    (Vec<Vec<T>>, Option<HashSet<U>>) where
    T: FromStr + Debug,
    U: FromStr + Debug + Hash + Eq {
    let file = if file.exists() {
      let ext = match file.extension() {
        Some(ext) => ext,
        None => panic!("Wrong file format, expected .libsvm")
      };
      if ext != "libsvm" {
        panic!("Wrong file format, expected .libsvm but found: {:?}", ext);
      }
      file
    } else {
      panic!("File {:?} doesn't exist.", file);
    };
    let file = match File::open(file) {
      Ok(file) => file,
      Err(err) => panic!("Error in opening file. {:?}", err)
    };
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    let mut data: Vec<Vec<T>> = Vec::new();
    let mut classes: Option<HashSet<U>> = None;
    if self.is_classif {
      classes = Some(HashSet::new());
    }
    let mut line_num = 1;
    while buf_reader.read_line(&mut line)
      .expect("Error in reading file") > 0 {
      line = line.trim().to_string();
      let mut values = line.split_whitespace();
      let mut sample: Vec<T> = Vec::new();
      let target = values.next();
      match target {
        Some(target) => {
          match target.parse::<T>() {
            Ok(target) => sample.push(target),
            Err(_) => panic!("Error in parsing target. Error in line: {:?}", line_num)
          }
          match classes {
            Some(ref mut classes) => {
              match target.parse::<U>() {
                Ok(target) => { classes.insert(target); },
                Err(_) => panic!("Error in parsing uniq classes. Error in line: {:?}", line_num)
              }
            },
            None => ()
          }
        },
        None => panic!("Error in parsing target. Error in line: {:?}", line_num)
      }
      for value in values {
        let feature: Vec<&str> = value.split(':').collect();
        let feature = *(feature.last().unwrap());
        match feature.parse::<T>() {
          Ok(f) => sample.push(f),
          Err(_) => panic!("Error in parsing feature. Error in line: {:?}", line_num)
        }
      }
      data.push(sample);
      line.clear();
      line_num += 1;
    }
    (data, classes)
  }

}