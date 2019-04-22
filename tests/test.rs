#[macro_use]
extern crate pretty_assertions;
extern crate libsvm_parser;

use std::path::Path;
use libsvm_parser::*;

#[test]
fn test_creating_exist() {
  let file_exist = Path::new("./tests/data/exist_empdty.libs0vm");
  let _parser = LIBSVMParser::new(file_exist);
}