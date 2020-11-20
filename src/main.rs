extern crate clap;

use core::fmt;
use fmt::Display;
use std::error::Error;
use std::io;

use chrono::{Duration, Local, TimeZone};
use serde::export::Formatter;
use tabular::{Row, Table};

use args::args;

use crate::args::Action;
use crate::data::{Data, load, store};

mod args;
mod data;

fn main() -> Result<(), io::Error> {
    let mut data = load()?;

    let result = match args() {
        Action::Start(tasks) => start(&mut data, tasks),
        Action::Stop(tasks) => stop(&mut data, tasks),
        Action::List => list(&data),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
    }

    store(data)?;
    Ok(())
}

fn start(data: &mut Data, tasks: Vec<String>) -> Result<(), PunchError> {
    let existing_tasks = tasks
        .iter()
        .filter(|t| data.tasks.contains_key(*t))
        .map(|t| t.clone())
        .collect::<Vec<_>>();

    if !existing_tasks.is_empty() {
        return Err(PunchError::ExistingTasks(existing_tasks));
    }

    for task in tasks {
        let now = Local::now();
        println!("Starting task `{}` at {}", task, now);
        data.tasks.insert(task, now.timestamp());
    }

    Ok(())
}

fn stop(data: &mut Data, tasks: Vec<String>) -> Result<(), PunchError> {
    let missing_tasks = tasks
        .iter()
        .filter(|t| !data.tasks.contains_key(*t))
        .map(|t| t.clone())
        .collect::<Vec<_>>();

    if !missing_tasks.is_empty() {
        return Err(PunchError::MissingTasks(missing_tasks));
    }

    for task in tasks {
        let timestamp = data.tasks.remove(&task).unwrap();
        let duration = PrettyDuration::new(&(Local::now() - Local.timestamp(timestamp, 0)));
        println!(
            "Stopping task `{}`, was running: {}",
            task,
            duration
        );
    }

    Ok(())
}

fn list(data: &Data) -> Result<(), PunchError> {
    let mut table = Table::new("{:<} {:>} {:>} {:>} {:>} {:>}");
    table.add_row(
        Row::new()
            .with_cell("Task")
            .with_cell("DateTime")
            .with_cell("Days")
            .with_cell("Hours")
            .with_cell("Minutes")
            .with_cell("Seconds"),
    );

    for (task, timestamp) in data.tasks.iter() {
        let start = Local.timestamp(*timestamp, 0);
        let duration = PrettyDuration::new(&(Local::now() - start));

        table.add_row(
            Row::new()
                .with_cell(task)
                .with_cell(start)
                .with_cell(duration.days)
                .with_cell(duration.hours)
                .with_cell(duration.minutes)
                .with_cell(duration.seconds),
        );
    }

    print!("{}", table);

    Ok(())
}

#[derive(Debug)]
enum PunchError {
    ExistingTasks(Vec<String>),
    MissingTasks(Vec<String>),
}

impl Error for PunchError {}

impl Display for PunchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PunchError::ExistingTasks(tasks) => {
                write!(f, "The following tasks already exist: {:?}", tasks)
            }
            PunchError::MissingTasks(tasks) => {
                write!(f, "The following tasks do not exist: {:?}", tasks)
            }
        }
    }
}

struct PrettyDuration {
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
