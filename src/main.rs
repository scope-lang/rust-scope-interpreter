use syntax::scope::Thing;
use std::env;
use std::fs::File;
use std::io::prelude::*;
mod syntax;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "scope.pest"]
struct ScopeParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    // --snip--
    let filename=&args[1];
    println!("In file {}", filename);
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
    let pairs = ScopeParser::parse(Rule::module, &contents).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());
        //println!("Stuff:   {:?}", pair);
        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.into_span().as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.into_span().as_str()),
                Rule::statement => println!("Statement:   {}", inner_pair.into_span().as_str()),
                _ => println!("????:   {}", inner_pair.as_str())
            };
        }
    }
    let test:syntax::scope::Object=syntax::scope::Object::empty();
    let value=syntax::scope::Number::new(10f64);
    test.setItem("x".to_owned(), &value)
}
