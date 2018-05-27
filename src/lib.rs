#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};
use std::{cmp, io::{self, BufReader, BufWriter, Write}, str::FromStr, time::Duration};

const HOURS: u64 = 60 * 60;
const LIGHTNING: u64 = 5 * 60;
const MORNING_SESSION_DURATION: Duration = Duration::from_secs(3 * HOURS);
const MORNING_SESSION_START: Duration = Duration::from_secs(9 * HOURS);
const EVENING_SESSION_MAX_DURATION: Duration = Duration::from_secs(4 * HOURS);
const EVENING_SESSION_START: Duration = Duration::from_secs(13 * HOURS);
const MORNING_EVENT: &'static str = "Lunch";
const EVENING_EVENT: &'static str = "Networking Event";

pub fn answers(input: impl io::Read, output: impl io::Write) -> Result<(), Error> {
    let input = BufReader::new(input);
    let mut output = BufWriter::new(output);

    let mut talks = parse_talks(input)?;
    talks.sort_by_key(|t| t.duration);

    let mut track_number = 1;
    let mut session = Vec::with_capacity(16);
    loop {
        let num_talks_before_schedule = talks.len();

        writeln!(output, "Track {}", track_number)?;
        for (session_start, session_length, session_event) in &[
            (
                &MORNING_SESSION_START,
                &MORNING_SESSION_DURATION,
                &MORNING_EVENT,
            ),
            (
                &EVENING_SESSION_START,
                &EVENING_SESSION_MAX_DURATION,
                &EVENING_EVENT,
            ),
        ] {
            session.clear();
            schedule(session_length, &mut talks, &mut session)?;
            format(session_start, &session, session_event, &mut output)?;
        }
        if num_talks_before_schedule == talks.len() {
            bail!("Could not schedule the remaining {} talks", talks.len())
        }
        if talks.is_empty() {
            return Ok(());
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
        talks.push(line.parse::<Talk>()
            .with_context(|_e| format!("Could not parse talk from '{}'", line))?);
    }
    Ok(talks)
}

fn format<'a>(
    start_at: &Duration,
    talks: &[Talk],
    special_event: &str,
    mut output: impl io::Write,
) -> Result<Duration, Error> {
    let mut current_time = *start_at;
    for talk in talks {
        let duration = if talk.duration.as_secs() == LIGHTNING {
            "lightning".to_string()
        } else {
            format!("{}min", talk.duration.as_secs() / 60)
        };
        writeln!(
            output,
            "{} {} {}",
            format_time(&current_time),
            talk.name,
            duration
        )?;
        current_time += talk.duration;
    }
    writeln!(output, "{} {}", format_time(&current_time), special_event)?;
    Ok(current_time)
}

fn format_time(time_of_day: &Duration) -> String {
    let hour_of_day = time_of_day.as_secs() / HOURS;
    let minutes_of_hour = (time_of_day.as_secs() % HOURS) / 60;
    let time_of_day = if hour_of_day >= 12 { "PM" } else { "AM" };
    format!("{:02}:{:02}{}", hour_of_day, minutes_of_hour, time_of_day)
}

/// from https://github.com/acmeism/RosettaCodeData/blob/master/Task/Knapsack-problem-0-1/Rust/knapsack-problem-0-1.rust
/// and adjusted accordingly
fn knapsack01_dyn(items: &[Talk], max_weight: usize, result: &mut Vec<Talk>) {
    let mut best_value = vec![vec![0; max_weight + 1]; items.len() + 1];
    for (i, it) in items.iter().enumerate() {
        for w in 1..max_weight + 1 {
            best_value[i + 1][w] = if it.weight() > w {
                best_value[i][w]
            } else {
                cmp::max(
                    best_value[i][w],
                    best_value[i][w - it.weight()] + it.value(),
                )
            }
        }
    }

    let mut left_weight = max_weight;

    for (i, it) in items.iter().enumerate().rev() {
        if best_value[i + 1][left_weight] != best_value[i][left_weight] {
            result.push(it.clone());
            left_weight -= it.weight();
        }
    }
}

fn schedule(
    session_duration: &Duration,
    talks: &mut Vec<Talk>,
    mut session: &mut Vec<Talk>,
) -> Result<(), Error> {
    knapsack01_dyn(&talks, session_duration.as_secs() as usize, &mut session);
    for session_talk in session {
        talks.retain(|t| t.name != session_talk.name)
    }
    Ok(())
}

#[derive(Clone)]
struct Talk {
    name: String,
    duration: Duration,
}

impl Talk {
    fn weight(&self) -> usize {
        self.duration.as_secs() as usize
    }
    fn value(&self) -> usize {
        self.weight()
    }
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
                duration: Duration::from_secs(LIGHTNING),
            }
        } else {
            bail!("Unknown talk format: '{}'", s)
        })
    }
}
