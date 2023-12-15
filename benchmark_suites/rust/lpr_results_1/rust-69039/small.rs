#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
fn main() {
    let mut d = Box::pin(|b| {
        let _c = yield "What is your name?";
        yield "How are you feeling?";
    });
    loop {
        match d.as_mut().resume(String::new()) {
            GeneratorState::Yielded(_f) => {}
            _ => {
                break;
            }
        }
        String::new();
    }
}
