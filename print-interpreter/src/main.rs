use std::fs;
use pest::Parser;


#[derive(pest_derive::Parser)]
#[grammar = "print.pest"]
pub struct PrintParser;

fn main() {
  let context = fs::read_to_string("input.txt")
      .expect("HEY, I CANNOT READ YOUR FILE LMAO");

  let _parsed = PrintParser::parse(Rule::print_statement, &context)
      .expect("Can't parse.");

  for pair in _parsed {
    for inner_pair in pair.into_inner() {
      println!("{}", inner_pair.as_str());
    }
  }
}