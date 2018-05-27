#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};
use std::{io::{self, BufRead, BufReader, BufWriter, Read}, str::FromStr, time::Duration};

pub fn answers(input: impl Read, output: impl io::Write) -> Result<(), Error> {
    let input = BufReader::new(input);
    let _output = BufWriter::new(output);

    let mut talks = Vec::new();
    for line in input.lines() {
        let line = line.with_context(|_e| "Could not read line from input")?;
        talks.push(line.parse::<Talk>()?);
    }

    unimplemented!()
}

struct Talk {
    name: String,
    duration: Duration,
}

impl FromStr for Talk {
    type Err = Error;

    fn from_str(s: &str) -> Result<Talk, Error> {
        if s.len() < "N XXmin".len() {
            bail!("Talk description is too short: '{}'", s)
        }
        Ok(if s.ends_with("min") {
            Talk {
                name: s[..s.len() - " XXmin".len()].to_owned(),
                duration: Duration::from_secs(
                    s[s.len() - "XXmin".len()..s.len() - "min".len()].parse::<u64>()? * 60,
                ),
            }
        } else if s.ends_with("lightning") {
            Talk {
                name: s[..s.len() - " lightning".len()].to_owned(),
                duration: Duration::from_secs(5 * 60),
            }
        } else {
            bail!("Could not parse talk from '{}'", s)
        })
    }
}
