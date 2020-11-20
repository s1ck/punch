use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

#[derive(Debug)]
pub enum Action {
    Start(Vec<String>),
    Stop(Vec<String>),
    List,
    History,
}

pub fn args() -> Action {
    match arg_matches().subcommand() {
        ("in", Some(args)) => Action::Start(tasks(args, "IN")),
        ("out", Some(args)) => Action::Stop(tasks(args, "OUT")),
        ("list", _) => Action::List,
        ("history", _) => Action::History,
        (action, _) => panic!("Invalid action: {}", action),
    }
}

fn tasks(args: &ArgMatches, arg_name: &str) -> Vec<String> {
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
        .subcommand(SubCommand::with_name("history").about("Print history of all tasks."))
        .get_matches()
}
