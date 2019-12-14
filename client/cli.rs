use ::std::env::current_dir;
use ::std::path::PathBuf;

use ::anyhow::{anyhow, Result};
use ::structopt::StructOpt;

use ::pueue::message::*;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// Queue a task for execution
    Add {
        /// The command that should be added
        #[structopt()]
        command: String,

        /// Start the task immediately
        #[structopt(name = "immediate", short, long)]
        start_immediately: bool,
    },
    /// Remove tasks from the list.
    /// You cannot remove running or paused tasks.
    Remove {
        /// The task ids to be removed
        task_ids: Vec<i32>,
    },
    /// Switches the queue position two commands. Only works on queued or stashed commands
    Switch {
        /// The first task id
        task_id_1: i32,
        /// The second task id
        task_id_2: i32,
    },
    /// Stash some tasks. These tasks won't be automatically started.
    /// Afterwards either `enqueue` them, to be normally handled or forcefully `start` them.
    Stash {
        /// The id(s) of the tasks you want to stash
        task_ids: Vec<i32>,
    },
    /// Enqueue stashed tasks. They'll be handled normally afterwards.
    Enqueue {
        /// The id(s) of the tasks you want to enqueue
        task_ids: Vec<i32>,
    },

    /// Wake the daemon from its paused state, including continuing all paused tasks.
    /// Does nothing if the daemon isn't paused.
    Start {
        /// Enforce starting these tasks.
        /// Doesn't affect the daemon or any other tasks.
        /// Works on a paused deamon.
        #[structopt(short, long)]
        task_ids: Option<Vec<i32>>,
    },
    Restart {
        /// Restart the
        #[structopt()]
        task_ids: Vec<i32>,

        /// Start the task(s) immediately
        #[structopt(name = "immediate", short, long)]
        start_immediately: bool,
    },
    /// Pause the daemon and all running tasks.
    /// A paused daemon won't start any new tasks.
    /// Daemon and tasks can be continued with `start`
    Pause {
        /// Pause the daemon, but let any running tasks finish by themselves.
        #[structopt(short, long, group("pause"), conflicts_with("task_ids"))]
        wait: bool,

        /// Enforce starting these tasks.
        /// Doesn't affect the daemon or any other tasks.
        #[structopt(short, long, group("pause"))]
        task_ids: Option<Vec<i32>>,
    },
    /// Pause the daemon and all running tasks.
    /// A paused daemon won't start any new tasks.
    /// Daemon and tasks can be continued with `start`
    Kill {
        /// Pause the daemon, but let any running tasks finish by themselves.
        #[structopt(short, long, group("kill"), conflicts_with("task_ids"))]
        all: bool,

        /// Enforce starting these tasks.
        /// Doesn't affect the daemon or any other tasks.
        #[structopt(group("kill"), required_unless("all"))]
        task_ids: Vec<i32>,
    },

    /// Send something to a task. For example, useful for sending confirmations ('y\n')
    Send {
        /// The id of the task
        task_id: i32,

        /// The input that should be sent to the process
        input: String,
    },
    /// Edit the command of a stashed or queued task.
    Edit {
        /// The id of the task
        task_id: i32,
    },

    /// Display the current status of all tasks
    Status {
        /// Print the current state as json to stdout
        /// This doesn't include stdout/stderr of tasks.
        /// Use `log -j` if you want everything
        #[structopt(short, long)]
        json: bool,
    },
    /// Display the log output of finished tasks
    Log {
        /// Specify for which specific tasks you want to see the output
        #[structopt(short, long)]
        task_ids: Option<Vec<i32>>,
        /// Print the current state as json
        /// Includes EVERYTHING
        #[structopt(short, long)]
        json: bool,
    },
    /// Show the output of a currently running task
    /// This command allows following (like `tail -f`)
    Show {
        /// The id of the task
        task_id: i32,
        /// Continuously print stdout (like `tail -f`)
        #[structopt(short, long)]
        follow: bool,
        /// Like -f, but shows stderr instead of stdeout.
        #[structopt(short, long)]
        err: bool,
    },
    /// Kill all running tasks, remove all tasks and reset max_id.
    Reset,
    /// Remove all finished tasks from the list (also clears logs).
    Clean,
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Pueue client",
    about = "Interact with the Pueue daemon",
    author = "Arne Beer <contact@arne.beer>"
)]
pub struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Optional custom config path
    #[structopt(name = "config", parse(from_os_str))]
    pub config_path: Option<PathBuf>,

    /// The url for the daemon. Overwrites the address in the config file
    #[structopt(short, long)]
    pub address: Option<String>,

    /// The port for the daemon. Overwrites the port in the config file
    #[structopt(short, long)]
    pub port: Option<String>,

    #[structopt(subcommand)]
    pub cmd: SubCommand,
}

// Convert and pre-process the sub-command into a valid message
// that can be understood by the daemon
pub fn get_message_from_opt(opt: &Opt) -> Result<Message> {
    match &opt.cmd {
        SubCommand::Add {
            command,
            start_immediately,
        } => {
            let cwd_pathbuf = current_dir()?;
            let cwd = cwd_pathbuf.to_str().ok_or(anyhow!(
                "Cannot parse current working directory (Invalid utf8?)"
            ))?;
            Ok(Message::Add(AddMessage {
                command: command.to_string(),
                path: cwd.to_string(),
                start_immediately: *start_immediately,
            }))
        }
        SubCommand::Remove { task_ids } => {
            let message = RemoveMessage {
                task_ids: task_ids.clone(),
            };
            Ok(Message::Remove(message))
        }
        SubCommand::Stash { task_ids } => {
            let message = StashMessage {
                task_ids: task_ids.clone(),
            };
            Ok(Message::Stash(message))
        }
        SubCommand::Switch { task_id_1, task_id_2 } => {
            let message = SwitchMessage {
                task_id_1: *task_id_1,
                task_id_2: *task_id_2,
            };
            Ok(Message::Switch(message))
        }
        SubCommand::Enqueue { task_ids } => {
            let message = EnqueueMessage {
                task_ids: task_ids.clone(),
            };
            Ok(Message::Enqueue(message))
        }
        SubCommand::Start { task_ids } => {
            let message = StartMessage {
                task_ids: task_ids.clone(),
            };
            Ok(Message::Start(message))
        }
        SubCommand::Restart {
            task_ids,
            start_immediately,
        } => {
            let message = RestartMessage {
                task_ids: task_ids.clone(),
                start_immediately: *start_immediately,
            };
            Ok(Message::Restart(message))
        }
        SubCommand::Pause { wait, task_ids } => {
            let message = PauseMessage {
                wait: *wait,
                task_ids: task_ids.clone(),
            };
            Ok(Message::Pause(message))
        }
        SubCommand::Kill { all, task_ids } => {
            let message = KillMessage {
                all: *all,
                task_ids: task_ids.clone(),
            };
            Ok(Message::Kill(message))
        }

        SubCommand::Send { task_id, input } => {
            let message = SendMessage {
                task_id: *task_id,
                input: input.clone(),
            };
            Ok(Message::Send(message))
        }
        SubCommand::Edit { task_id } => {
            let message = EditRequestMessage { task_id: *task_id };
            Ok(Message::EditRequest(message))
        }

        SubCommand::Status{ json: _ } => Ok(Message::SimpleStatus),
        SubCommand::Log { task_ids: _, json: _ } => Ok(Message::Status),
        SubCommand::Show { task_id, follow, err } => {
            let message = StreamRequestMessage {
                task_id: *task_id,
                follow: *follow,
                err: *err,
            };
            Ok(Message::StreamRequest(message))
        },
        SubCommand::Clean => Ok(Message::Clean),
        SubCommand::Reset => Ok(Message::Reset),
    }
}