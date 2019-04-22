extern crate termcolor;

use std::path::Path;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug)]
pub struct LIBSVMParser<'libsvm> {
  file: &'libsvm Path
}

fn print_error_msg(msg: &str) {
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
  write!(&mut stdout, "[LIBSVM PARSER ERROR]").unwrap();
  stdout.reset().unwrap();
  println!(": 🚫   {}", msg);
}

impl<'libsvm> LIBSVMParser<'libsvm> {
  pub fn new(file: &'libsvm Path) -> Self {
    if file.exists() {
      let ext = match file.extension() {
        Some(ext) => ext,
        None => {
          print_error_msg("Wrong file format, expected .libsvm!");
          panic!();
        }
      };
      if ext != "libsvm" {
        print_error_msg(&format!("Wrong file format, expected .libsvm, got .{:?}", ext));
        panic!();
      }
      Self {
        file: Path::new(file)
      }
    } else {
      print_error_msg(&format!("File {:?} doesn't exist.", file));
      panic!();
    }
  }
}