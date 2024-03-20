use crate::sys::Selector;
use crate::Token;
use std::io;

#[derive(Debug)]
pub struct Waker {}

impl Waker {
    pub fn new(_: &Selector, _: Token) -> io::Result<Waker> {
        Ok(Waker {}) // TODO
    }

    pub fn wake(&self) -> io::Result<()> {
        Ok(()) // TODO
    }
}
