extern crate clap;

use core::fmt;
use fmt::Display;
use std::error::Error;
use std::io;

use chrono::{Duration, Local, TimeZone};
use serde::export::Formatter;

use args::args;

use crate::args::Action;
use crate::data::{Data, load, store};

mod args;
mod data;

fn main() -> Result<(), io::Error> {
    let mut data = load()?;

    let result = match args() {
        Action::START(tasks) => start(&mut data, tasks),
        Action::STOP(tasks) => stop(&mut data, tasks),
        Action::LIST => list(&data),
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
        return Err(PunchError::ExistingTasks(existing_tasks))
    }

    for task in tasks {
        let dt = Local::now();
        println!("Starting task `{}` at {}", task, dt);
        data.tasks.insert(task, dt.timestamp());
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
        return Err(PunchError::MissingTasks(missing_tasks))
    }

    for task in tasks {
        let timestamp = data.tasks.remove(&task).unwrap();
        let duration = Local::now() - Local.timestamp(timestamp, 0);
        println!(
            "Stopping task `{}`, was running: {}",
            task,
            pretty_duration(&duration)
        );
    }

    Ok(())
}

fn list(data: &Data) -> Result<(), PunchError> {
    for (task, timestamp) in data.tasks.iter() {
        let duration = Local::now() - Local.timestamp(*timestamp, 0);

        println!(
            "Task `{}` is running since: {}",
            task,
            pretty_duration(&duration)
        );
    }
    Ok(())
}

fn pretty_duration(duration: &Duration) -> String {
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

    format!(
        "{} days, {} hours, {} minutes, {} seconds",
        days, hours, minutes, seconds
    )
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
            PunchError::ExistingTasks(tasks) => write!(f, "The following tasks already exist: {:?}", tasks),
            PunchError::MissingTasks(tasks) => write!(f, "The following tasks do not exist: {:?}", tasks),
        }
    }
}
