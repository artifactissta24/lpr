#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
fn main() {
    let mut d = Box::pin(|b| {
        let name_prompt = yield "What is your name?";
        yield "How are you feeling?";
    });
    loop {
        let input = String::new();
        match d.as_mut().resume(input) {
            GeneratorState::Yielded(prompt) => if prompt == "What is your name?" {},
            _ => break,
        }
    }
}
