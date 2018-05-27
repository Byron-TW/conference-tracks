#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};
use std::{io::{self, BufRead, BufReader, BufWriter, Read}, str::FromStr, time::Duration};
use std::iter::once;

pub fn answers(input: impl Read, output: impl io::Write) -> Result<(), Error> {
    let input = BufReader::new(input);
    let mut output = BufWriter::new(output);

    let mut talks = Vec::new();
    for line in input.lines() {
        let line = line.with_context(|_e| "Could not read line from input")?;
        talks.push(line.parse::<Talk>()?);
    }
    talks.sort_by(|a, b| b.duration.cmp(&a.duration));

    const MORNING_SESSION_DURATION: Duration = Duration::from_secs(3 * 60 * 60);
    const MORNING_SESSION_START: Duration = Duration::from_secs(9 * 60 * 60);
    const EVENING_SESSION_DURATION: Duration = Duration::from_secs(4 * 60 * 60);
    const EVENING_SESSION_START: Duration = Duration::from_secs(13 * 60 * 60);

    let mut track_number = 1;
    let mut session = Vec::with_capacity(16);
    let lunch = Talk {
        name: "Lunch".to_owned(),
        duration: Duration::from_secs(60 * 60),
    };
    while !talks.is_empty() {
        let num_talks_before_schedule = talks.len();
        writeln!(output, "Track {}", track_number)?;
        for (session_start, session_length) in &[
            (MORNING_SESSION_START, MORNING_SESSION_DURATION),
            (EVENING_SESSION_START, EVENING_SESSION_DURATION),
        ] {
            session.clear();
            schedule(session_length, &mut talks, &mut session)?;
            format(
                session_start,
                session.iter().chain(once(&lunch)),
                &mut output,
            )?;
        }
        if num_talks_before_schedule == talks.len() {
            bail!("Could not schedule the remaining {} talks", talks.len())
        }
        track_number += 1;
    }
    Ok(())
}

fn format<'a>(
    start_at: &Duration,
    talks: impl Iterator<Item = &'a Talk>,
    output: impl io::Write,
) -> Result<(), Error> {
    unimplemented!()
}

fn schedule(
    _duration: &Duration,
    talks: &mut Vec<Talk>,
    session: &mut Vec<Talk>,
) -> Result<(), Error> {
    let mut c = 10;

    while let Some(talk) = talks.pop() {
        session.push(talk);
        c -= 1;
        if c == 0 {
            return Ok(());
        }
    }
    Ok(())
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
