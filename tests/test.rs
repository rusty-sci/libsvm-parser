#[macro_use]
extern crate pretty_assertions;
extern crate libsvm_parser;

use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;
use libsvm_parser::*;

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
  let parser = LIBSVMParser::new();
  let (result, _) = parser.parse_file::<f32, i32>(file_exist);

  for (i, row) in data.iter().enumerate() {
    for (j, value) in row.iter().enumerate() {
      assert_eq!(*value, result[i][j]);
    }
  }
}

#[test]
fn test_read_file_for_classif() {
  let data = vec![vec![0.0, 5.1, 3.5, 1.4, 0.2],
                  vec![1.0, 4.9, 3.0, 1.4, 0.2],
                  vec![0.0, 4.7, 3.2, 1.3, 0.2],
                  vec![2.0, 4.6, 3.1, 1.5, 0.2],
                  vec![3.0, 5.0, 3.6, 1.4, 0.2]];
  let classes: Vec<i32> = vec![0, 1, 0, 2, 3];
  let classes_set: HashSet<i32> = HashSet::from_iter(classes.iter().cloned());
  let file_exist = Path::new("./tests/data/test1.libsvm");
  let parser = LIBSVMParser::new().is_classification(true);
  let (result, res_classes) = parser.parse_file::<f32, i32>(file_exist);
  let res_classes = res_classes.unwrap();
  for (i, row) in data.iter().enumerate() {
    for (j, value) in row.iter().enumerate() {
      assert_eq!(*value, result[i][j]);
    }
  }
  let diff: HashSet<_> = res_classes.difference(&classes_set).collect();
  assert!(diff.is_empty());
}