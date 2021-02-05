use std::fmt;
use std::iter;

#[derive(Debug)]
struct Foo(u32);

#[derive(Debug)]
enum Error {
    Boom,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Boom => write!(f, "BOOM!")
        }
    }
}

impl Foo {
    #[tracing::instrument(skip(self), err)]
    fn test(&self) -> Result<impl Iterator<Item = u32>, Error> {
        Ok(iter::repeat(self.0))
    }
}

fn main() {
    for n in Foo(10).test().unwrap().take(5) {
        println!("Hello, {}!", n);
    }
}
