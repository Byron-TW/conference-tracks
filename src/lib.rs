extern crate failure;

use failure::Error;
use std::io::{self, BufReader, BufWriter, Read};

pub fn answers(input: impl Read, output: impl io::Write) -> Result<(), Error> {
    let _input = BufReader::new(input);
    let _output = BufWriter::new(output);
    unimplemented!()
}
