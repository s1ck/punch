use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub enum Action {
    Start(Vec<String>),
    Stop(Vec<String>),
    List,
    History(HistoryMode),
}

pub enum HistoryMode {
    Sum,
    Average,
}

pub fn args() -> Action {
    match arg_matches().subcommand() {
        ("in", Some(args)) => Action::Start(tasks(args, "IN")),
        ("out", Some(args)) => Action::Stop(tasks(args, "OUT")),
        ("list", _) => Action::List,
        ("history", Some(args)) if args.is_present("average") => {
            Action::History(HistoryMode::Average)
        }
        ("history", _) => Action::History(HistoryMode::Sum),
        (_, _) => unreachable!(),
    }
}

fn tasks(args: &ArgMatches<'_>, arg_name: &str) -> Vec<String> {
    args.values_of(arg_name)
        .map(|v| v.map(|s| s.to_string()).collect::<Vec<_>>())
        .unwrap_or_default()
}

fn arg_matches() -> ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("in")
                .about("Start working on one or more tasks.")
                .arg(
                    Arg::with_name("IN")
                        .value_name("task")
                        .help("Start working on the specified task(s).")
                        .min_values(1)
                        .multiple(false)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("out")
                .about("Stop working on one or more tasks.")
                .arg(
                    Arg::with_name("OUT")
                        .value_name("task")
                        .help("Stop working on the specified task(s).")
                        .min_values(1)
                        .multiple(false)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List all running tasks."))
        .subcommand(
            SubCommand::with_name("history")
                .about("Print history of all tasks.")
                .arg(
                    Arg::with_name("sum")
                        .long("total")
                        .short("t")
                        .help("Print the sum of durations for each task.")
                        .required(false),
                )
                .arg(
                    Arg::with_name("average")
                        .long("average")
                        .short("a")
                        .help("Print the average of durations for each task.")
                        .required(false),
                ),
        )
        .get_matches()
}
