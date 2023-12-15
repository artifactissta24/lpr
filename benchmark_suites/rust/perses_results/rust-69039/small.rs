#![feature(generators, generator_trait)]
use std::{
    ops::{Generator, GeneratorState}
};
fn my_scenario() -> impl Generator<String                                       > {
    |_arg        | {
        let my_name = yield "What is your name?";
                      yield "How are you feeling?";
    }
}
fn main() {
    let mut my_session = Box::pin(my_scenario());
    loop {
        let     line = String::new();
        match my_session.as_mut().resume(line) {
            GeneratorState::Yielded(prompt) => {
            }
                            Complete    => {
                break;
            }
        }
               String::new();
    }
}
