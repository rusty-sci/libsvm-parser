#[macro_use]
extern crate pretty_assertions;
extern crate libsvm_parser;

use std::path::Path;
use libsvm_parser::*;

#[test]
fn test_creating_exist() {
  let file_exist = Path::new("./tests/data/exist_empty.libsvm");
  let _parser = LIBSVMParser::new(file_exist);
}

#[test]
fn test_read_file_by_line() {
  let data = vec![vec![0.0, 5.1, 3.5, 1.4, 0.2],
                  vec![0.0, 4.9, 3.0, 1.4, 0.2],
                  vec![0.0, 4.7, 3.2, 1.3, 0.2],
                  vec![1.0, 4.6, 3.1, 1.5, 0.2],
                  vec![1.0, 5.0, 3.6, 1.4, 0.2],
                  vec![34.14, 5.4, 3.9, 1.7, 0.4],
                  vec![1.4, 4.6, 3.4, 1.4, 0.3],
                  vec![2.0, 5.0, 3.4, 1.5, 0.2],
                  vec![4.0, 4.4, 2.9, 1.4, 0.2],
                  vec![-6.0, 4.9, 3.1, 1.5, 0.1]];
  let file_exist = Path::new("./tests/data/test.libsvm");
  let parser = LIBSVMParser::new(file_exist);
  let result = parser.read_file::<f32>();

  for (i, row) in data.iter().enumerate() {
    for (j, value) in row.iter().enumerate() {
      assert_eq!(*value, result[i][j]);
    }
  }
}