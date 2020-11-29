/*!
A simple time clock tool.

# Usage

```bash

# Start working on tasks
punch in task1 [task2...]

# List all running tasks
punch list

# Stop working on tasks
punch out task1 [task2...]

# Print durations of all previous tasks
punch history [--average|--total]
```

*/
extern crate clap;

use std::error::Error;
use std::fmt;
use std::io;

use chrono::{Local, TimeZone};

use crate::args::args;
use crate::args::Action;
use crate::data::{load, store, Data, PrettyDuration};

mod args;
mod data;

fn main() -> Result<(), io::Error> {
    let mut data = load()?;

    let result = match args() {
        Action::Start(tasks) => start(&mut data, tasks),
        Action::Stop(tasks) => stop(&mut data, tasks),
        Action::List => {
            print!("{}", data.table_running());
            Ok(())
        }
        Action::History(kind) => {
            print!("{}", data.table_history(kind));
            Ok(())
        }
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
        .filter(|t| data.running.contains_key(*t))
        .map(|t| t.clone())
        .collect::<Vec<_>>();

    if !existing_tasks.is_empty() {
        return Err(PunchError::ExistingTasks(existing_tasks));
    }

    for task in tasks {
        let now = Local::now();
        println!("Starting task `{}` at {}", task, now);
        data.running.insert(task, now.timestamp());
    }

    Ok(())
}

fn stop(data: &mut Data, tasks: Vec<String>) -> Result<(), PunchError> {
    let missing_tasks = tasks
        .iter()
        .filter(|t| !data.running.contains_key(*t))
        .map(|t| t.clone())
        .collect::<Vec<_>>();

    if !missing_tasks.is_empty() {
        return Err(PunchError::MissingTasks(missing_tasks));
    }

    for task in tasks {
        let timestamp = data.running.remove(&task).unwrap();
        let duration = Local::now() - Local.timestamp(timestamp, 0);
        let durations = data.history.entry(task.clone()).or_default();
        durations.push(duration.num_seconds());

        let duration = PrettyDuration::new(&duration);
        println!("Stopped task `{}` after {}", task, duration);
    }

    Ok(())
}

#[derive(Debug)]
enum PunchError {
    ExistingTasks(Vec<String>),
    MissingTasks(Vec<String>),
}

impl Error for PunchError {}

impl fmt::Display for PunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
