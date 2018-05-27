#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};
use std::{io::{self, BufReader, BufWriter, Write}, str::FromStr, time::Duration};
use std::iter::once;

const HOURS: u64 = 60 * 60;
const MORNING_SESSION_DURATION: Duration = Duration::from_secs(3 * HOURS);
const MORNING_SESSION_START: Duration = Duration::from_secs(9 * HOURS);
const EVENING_SESSION_MAX_DURATION: Duration = Duration::from_secs(4 * HOURS);
const EVENING_SESSION_START: Duration = Duration::from_secs(13 * HOURS);

pub fn answers(input: impl io::Read, output: impl io::Write) -> Result<(), Error> {
    let input = BufReader::new(input);
    let mut output = BufWriter::new(output);

    let mut talks = parse_talks(input)?;
    talks.sort_by_key(|t| t.duration);

    let morning_event = Talk {
        name: "Lunch".to_owned(),
        duration: Duration::from_secs(1 * HOURS),
    };
    let evening_event = Talk {
        name: "Networking Event".to_owned(),
        duration: Duration::from_secs(1 * HOURS),
    };

    let mut track_number = 1;
    let mut session = Vec::with_capacity(16);
    loop {
        let num_talks_before_schedule = talks.len();

        writeln!(output, "Track {}", track_number)?;
        for (session_start, session_length, session_event) in &[
            (
                &MORNING_SESSION_START,
                &MORNING_SESSION_DURATION,
                &morning_event,
            ),
            (
                &EVENING_SESSION_START,
                &EVENING_SESSION_MAX_DURATION,
                &evening_event,
            ),
        ] {
            session.clear();
            schedule(session_length, &mut talks, &mut session)?;
            format(
                session_start,
                session.iter().chain(once(*session_event)),
                &mut output,
            )?;
        }
        if num_talks_before_schedule == talks.len() {
            bail!("Could not schedule the remaining {} talks", talks.len())
        }
        if talks.is_empty() {
            return Ok(())
        } else {
            write!(output, "\n\n")?;
        }
        track_number += 1;
    }
}

fn parse_talks(input: impl io::BufRead) -> Result<Vec<Talk>, Error> {
    let mut talks = Vec::new();
    for line in input.lines() {
        let line = line.with_context(|_e| "Could not read line from input")?;
        talks.push(line.parse::<Talk>()?);
    }
    Ok(talks)
}

fn format<'a>(
    start_at: &Duration,
    talks: impl Iterator<Item = &'a Talk>,
    mut output: impl io::Write,
) -> Result<Duration, Error> {
    let mut current_time = *start_at;
    for talk in talks {
        let hour_of_day = current_time.as_secs() / HOURS;
        let minutes_of_hour = (current_time.as_secs() % HOURS) / 60;
        let time_of_day = if hour_of_day >= 12 { "PM" } else { "AM" };
        writeln!(
            output,
            "{} {:02}:{:02}{}",
            talk.name, hour_of_day, minutes_of_hour, time_of_day
        )?;
        current_time += talk.duration;
    }
    Ok(current_time)
}

fn schedule(
    _session_duration: &Duration,
    talks: &mut Vec<Talk>,
    session: &mut Vec<Talk>,
) -> Result<(), Error> {
    let mut c = 5;

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
