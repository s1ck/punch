use core::fmt;
use std::{fs, io};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use chrono::{Duration, Local, TimeZone};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tabular::{Row, Table};

const DATA_FILE: &str = "data.json";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Data {
    // task -> creation date in seconds
    pub running: HashMap<String, i64>,
    // task -> duration in seconds
    pub history: HashMap<String, i64>,
}

impl Data {
    pub fn table_running(&self) -> Table {
        Data::table(self.running.iter(), |timestamp| {
            Local::now() - Local.timestamp(timestamp, 0)
        })
    }

    pub fn table_history(&self) -> Table {
        // Sort tasks by descending duration
        let mut tasks = self.history.iter().collect::<Vec<_>>();
        tasks.sort_by_key(|&(_, v)| -v);
        Data::table(tasks, Duration::seconds)
    }

    fn table<'a, F, I>(tuples: I, duration_fn: F) -> Table
        where
            I: IntoIterator<Item=(&'a String, &'a i64)>,
            F: Fn(i64) -> Duration,
    {
        let mut table = Table::new("{:<} {:>} {:>} {:>} {:>}");
        table.add_row(
            Row::new()
                .with_cell("Task")
                .with_cell("Days")
                .with_cell("Hours")
                .with_cell("Minutes")
                .with_cell("Seconds"),
        );

        for (task, timestamp) in tuples {
            let duration = PrettyDuration::new(&duration_fn(*timestamp));

            table.add_row(
                Row::new()
                    .with_cell(task)
                    .with_cell(duration.days)
                    .with_cell(duration.hours)
                    .with_cell(duration.minutes)
                    .with_cell(duration.seconds),
            );
        }

        table
    }
}

pub struct PrettyDuration {
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
}

impl PrettyDuration {
    pub fn new(duration: &Duration) -> Self {
        let days = duration.num_days();
        let hours = if duration.num_hours() >= 24 {
            duration.num_hours() % 24
        } else {
            duration.num_hours()
        };
        let minutes = if duration.num_minutes() >= 60 {
            duration.num_minutes() % 60
        } else {
            duration.num_minutes()
        };
        let seconds = if duration.num_seconds() >= 60 {
            duration.num_seconds() % 60
        } else {
            duration.num_seconds()
        };

        PrettyDuration {
            days,
            hours,
            minutes,
            seconds,
        }
    }
}

impl Display for PrettyDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} days, {} hours, {} minutes, {} seconds",
            self.days, self.hours, self.minutes, self.seconds
        )
    }
}

pub fn load() -> Result<Data, io::Error> {
    let file = match data_file() {
        Some(file) => file,
        None => return Ok(Data::default()),
    };

    match File::open(file) {
        Ok(file) => Ok(load_inner(file)?),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(Data::default()),
            kind => Err(Error::from(kind)),
        }
    }
}

fn load_inner<R: io::Read>(source: R) -> Result<Data, io::Error> {
    Ok(serde_json::from_reader(source)?)
}

pub fn store(data: Data) -> Result<(), io::Error> {
    let file = data_file().ok_or(Error::new(ErrorKind::NotFound, "No data directory."))?;

    // Create parent directory if it does not exist, yet
    if let Some(parent) = file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let file = File::create(file)?;
    store_inner(data, file)?;
    Ok(())
}

fn store_inner<W: io::Write>(data: Data, target: W) -> Result<(), io::Error> {
    Ok(serde_json::to_writer_pretty(target, &data)?)
}

fn data_file() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("de", "s1ck", env!("CARGO_PKG_NAME"))?;
    let mut file = dirs.data_dir().to_path_buf();
    file.push(DATA_FILE);
    Some(file)
}
