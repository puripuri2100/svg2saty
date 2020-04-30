extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::Write;
use svg;

pub mod body;
pub mod make_satysfi_file_text;

fn write_file(file_name: String, text: String) -> () {
  let mut file = File::create(file_name).unwrap();
  file.write_all(text.as_bytes()).unwrap();
}

fn main() {
  let app = App::new("clapex")
    .version("0.1.0")
    .arg(
      Arg::with_name("output")
        .help("Specify output file")
        .short("o")
        .long("output")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("input")
        .help("Specify output file")
        .short("i")
        .long("input")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("fun_name")
        .help("Specify fucntion name")
        .short("n")
        .long("name")
        .takes_value(true),
    );

  let matches = app.get_matches();

  let mut output_name: Option<String> = None;
  let mut input_name: Option<String> = None;
  let mut fun_name: Option<String> = None;

  if let Some(output) = matches.value_of("output") {
    output_name = Some(output.to_string());
    print!("Value for output: {}\n", output);
  }

  if let Some(input) = matches.value_of("input") {
    input_name = Some(input.to_string());
    print!("Value for input: {}\n", input);
  }

  if let Some(fun) = matches.value_of("fun_name") {
    fun_name = Some(fun.to_string());
    print!("Value for funciton name: {}\n", fun);
  }

  let _ = match (&mut input_name, &mut output_name, &mut fun_name) {
    (Some(i), Some(o), Some(f)) => {
      let package_list = vec!["gr", "base/float"];
      let svg = svg::open(i).unwrap();
      let body = body::main(svg);
      let s = make_satysfi_file_text::make_satysfi_file_text(package_list, f, &body);
      write_file(o.to_string(), s)
    }
    (_, _, _) => (),
  };
}
