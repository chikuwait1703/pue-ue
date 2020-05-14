use ::anyhow::Result;
use ::comfy_table::presets::UTF8_HORIZONTAL_BORDERS_ONLY;
use ::comfy_table::*;
use ::crossterm::style::style;
use ::snap::read::FrameDecoder;
use ::std::collections::BTreeMap;
use ::std::io;
use ::std::string::ToString;

use ::pueue::log::{get_log_file_handles, get_log_paths};
use ::pueue::message::TaskLogMessage;
use ::pueue::settings::Settings;
use ::pueue::state::State;
use ::pueue::task::{Task, TaskResult, TaskStatus};

use crate::cli::SubCommand;
use crate::output_helper::*;

pub fn print_success(message: String) {
    println!("{}", message);
}

pub fn print_error(message: String) {
    let styled = style(message).with(Color::Red);
    println!("{}", styled);
}

/// Print the current state of the daemon in a nicely formatted table.
pub fn print_state(state: State, cli_command: &SubCommand) {
    let (json, group_only) = match cli_command {
        SubCommand::Status { json, group } => (*json, group.clone()),
        _ => panic!(
            "Got wrong Subcommand {:?} in print_state. This shouldn't happen",
            cli_command
        ),
    };

    // If the json flag is specified, print the state as json and exit.
    if json {
        println!("{}", serde_json::to_string(&state).unwrap());
        return;
    }

    // Don't show default queue headline if a single group is requested
    if group_only.is_none() {
        println!("{}", get_default_headline(&state));
    }

    // Early exit and hint if there are no tasks in the queue
    if state.tasks.is_empty() {
        println!("\nTask list is empty. Add tasks with `pueue add -- [cmd]`");
        return;
    }

    // Skip default queue, if a single group is requested
    if group_only.is_none() {
        let default_tasks = get_default_tasks(&state.tasks);
        if !default_tasks.is_empty() {
            print_table(&default_tasks);
        }
    }

    // Print new table for each group
    for (group, tasks) in sort_tasks_by_group(&state.tasks) {
        // Skip unwanted groups, if a single group is requested
        if let Some(group_only) = &group_only {
            if group_only != &group {
                continue;
            }
        }
        println!("{}", get_group_headline(&group, &state));
        print_table(&tasks);
    }
}

/// Print some tasks into a nicely formatted table
fn print_table(tasks: &BTreeMap<usize, Task>) {
    let (has_delayed_tasks, has_dependencies) = has_special_columns(tasks);

    // Create table header row
    let mut headers = vec![Cell::new("Index"), Cell::new("Status")];
    if has_delayed_tasks {
        headers.push(Cell::new("Enqueue At"));
    }
    if has_dependencies {
        headers.push(Cell::new("Deps"));
    }
    headers.append(&mut vec![
        Cell::new("Exitcode"),
        Cell::new("Command"),
        Cell::new("Path"),
        Cell::new("Start"),
        Cell::new("End"),
    ]);

    // Initialize comfy table.
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .load_preset(UTF8_HORIZONTAL_BORDERS_ONLY)
        .set_header(headers);

    // Add rows one by one.
    for (id, task) in tasks {
        let mut row = Row::new();
        row.add_cell(Cell::new(&id.to_string()));

        // Determine the human readable task status representation and the respective color.
        let status_string = task.status.to_string();
        let (status_text, color) = match task.status {
            TaskStatus::Running => (status_string, Color::Green),
            TaskStatus::Paused | TaskStatus::Locked => (status_string, Color::White),
            TaskStatus::Done => match &task.result {
                Some(TaskResult::Success) => (TaskResult::Success.to_string(), Color::Green),
                Some(TaskResult::DependencyFailed) => ("Dependency failed".to_string(), Color::Red),
                Some(TaskResult::FailedToSpawn(_)) => ("Failed to spawn".to_string(), Color::Red),
                Some(result) => (result.to_string(), Color::Red),
                None => panic!("Got a 'Done' task without a task result. Please report this bug."),
            },
            _ => (status_string, Color::Yellow),
        };
        row.add_cell(Cell::new(status_text).fg(color));

        if has_delayed_tasks {
            if let Some(enqueue_at) = task.enqueue_at {
                row.add_cell(Cell::new(enqueue_at.format("%Y-%m-%d\n%H:%M:%S")));
            } else {
                row.add_cell(Cell::new(""));
            }
        }

        if has_dependencies {
            let text = task
                .dependencies
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            row.add_cell(Cell::new(text));
        }

        // Match the color of the exit code.
        // If the exit_code is none, it has been killed by the task handler.
        let exit_code_cell = match task.result {
            Some(TaskResult::Success) => Cell::new("0").fg(Color::Green),
            Some(TaskResult::Failed(code)) => Cell::new(&code.to_string()).fg(Color::Red),
            _ => Cell::new(""),
        };
        row.add_cell(exit_code_cell);

        // Add command and path.
        row.add_cell(Cell::new(&task.command));
        row.add_cell(Cell::new(&task.path));

        // Add start time, if already set.
        if let Some(start) = task.start {
            let formatted = start.format("%H:%M").to_string();
            row.add_cell(Cell::new(&formatted));
        } else {
            row.add_cell(Cell::new(""));
        }

        // Add finish time, if already set.
        if let Some(end) = task.end {
            let formatted = end.format("%H:%M").to_string();
            row.add_cell(Cell::new(&formatted));
        } else {
            row.add_cell(Cell::new(""));
        }

        table.add_row(row);
    }

    // Print the table.
    println!("{}", table);
}

/// Print the log ouput of finished tasks.
/// Either print the logs of every task
/// or only print the logs of the specified tasks.
pub fn print_logs(
    mut task_logs: BTreeMap<usize, TaskLogMessage>,
    cli_command: &SubCommand,
    settings: &Settings,
) {
    let (json, task_ids) = match cli_command {
        SubCommand::Log { json, task_ids } => (*json, task_ids.clone()),
        SubCommand::Follow { task_id, .. } => (false, vec![*task_id]),
        _ => panic!(
            "Got wrong Subcommand {:?} in print_log. This shouldn't happen",
            cli_command
        ),
    };
    if json {
        println!("{}", serde_json::to_string(&task_logs).unwrap());
        return;
    }

    if task_ids.is_empty() && task_logs.is_empty() {
        println!("There are no finished tasks");
        return;
    }

    if !task_ids.is_empty() && task_logs.is_empty() {
        println!("There are no finished tasks for your specified ids");
        return;
    }

    let mut task_iter = task_logs.iter_mut().peekable();
    while let Some((_, mut task_log)) = task_iter.next() {
        print_log(&mut task_log, settings);

        // Add a newline if there is another task that's going to be printed.
        if let Some((_, task_log)) = task_iter.peek() {
            if !vec![TaskStatus::Done, TaskStatus::Running, TaskStatus::Paused]
                .contains(&task_log.task.status)
            {
                println!();
            }
        }
    }
}

/// Print the log of a single task.
pub fn print_log(task_log: &mut TaskLogMessage, settings: &Settings) {
    let task = &task_log.task;
    // We only show logs of finished or running tasks.
    if !vec![TaskStatus::Done, TaskStatus::Running, TaskStatus::Paused].contains(&task.status) {
        return;
    }

    // Print task id and exit code.
    let task_text = style(format!("Task {} ", task.id)).attribute(Attribute::Bold);
    let exit_status = match &task.result {
        Some(TaskResult::Success) => style("with exit code 0".into()).with(Color::Green),
        Some(TaskResult::Failed(exit_code)) => {
            style(format!("with exit code {}", exit_code)).with(Color::Red)
        }
        Some(TaskResult::FailedToSpawn(err)) => {
            style(format!("failed to spawn: {}", err)).with(Color::Red)
        }
        Some(TaskResult::Killed) => style("killed by system or user".to_string()).with(Color::Red),
        Some(TaskResult::DependencyFailed) => {
            style("dependency failed".to_string()).with(Color::Red)
        }
        None => style("running".to_string()),
    };
    print!("{} {}", task_text, exit_status);

    // Print command and path.
    println!("Command: {}", task.command);
    println!("Path: {}", task.path);

    if let Some(start) = task.start {
        println!("Start: {}", start.to_rfc2822());
    }
    if let Some(end) = task.end {
        println!("End: {}", end.to_rfc2822());
    }

    if settings.client.read_local_logs {
        print_local_log_output(task_log.task.id, settings);
    } else if task_log.stdout.is_some() && task_log.stderr.is_some() {
        print_task_output_from_daemon(task_log);
    } else {
        println!("Logs requested from pueue daemon, but none received. Please report this bug.");
    }
}

/// The daemon didn't send any log output, thereby we didn't request any.
/// If that's the case, read the log files from the local pueue directory
pub fn print_local_log_output(task_id: usize, settings: &Settings) {
    let (mut stdout_log, mut stderr_log) =
        match get_log_file_handles(task_id, &settings.daemon.pueue_directory) {
            Ok((stdout, stderr)) => (stdout, stderr),
            Err(err) => {
                println!("Failed to get log file handles: {}", err);
                return;
            }
        };
    // Stdout handler to directly write log file output to io::stdout
    // without having to load anything into memory.
    let mut stdout = io::stdout();

    if let Ok(metadata) = stdout_log.metadata() {
        if metadata.len() != 0 {
            println!(
                "{}",
                style("stdout:")
                    .with(Color::Green)
                    .attribute(Attribute::Bold)
            );

            if let Err(err) = io::copy(&mut stdout_log, &mut stdout) {
                println!("Failed reading local stdout log file: {}", err);
            };
        }
    }

    if let Ok(metadata) = stderr_log.metadata() {
        println!(
            "{}",
            style("stderr:")
                .with(Color::Green)
                .attribute(Attribute::Bold)
        );

        if metadata.len() != 0 {
            if let Err(err) = io::copy(&mut stderr_log, &mut stdout) {
                println!("Failed reading local stderr log file: {}", err);
            };
        }
    }
}

/// Prints log output received from the daemon.
/// We can safely call .unwrap() on stdout and stderr in here, since this
/// branch is always called after ensuring that both are `Some`.
pub fn print_task_output_from_daemon(task_log: &TaskLogMessage) {
    if !task_log.stdout.as_ref().unwrap().is_empty() {
        if let Err(err) = print_remote_task_output(&task_log, true) {
            println!("Error while parsing stdout: {}", err);
        }
    }

    if !task_log.stderr.as_ref().unwrap().is_empty() {
        if let Err(err) = print_remote_task_output(&task_log, false) {
            println!("Error while parsing stderr: {}", err);
        };
    }
}

/// Print log output of a finished process.
pub fn print_remote_task_output(task_log: &TaskLogMessage, stdout: bool) -> Result<()> {
    let (pre_text, bytes) = if stdout {
        ("stdout: ", task_log.stdout.as_ref().unwrap())
    } else {
        ("stderr: ", task_log.stderr.as_ref().unwrap())
    };

    println!(
        "{}",
        style(pre_text)
            .with(Color::Green)
            .attribute(Attribute::Bold)
    );

    let mut decompressor = FrameDecoder::new(bytes.as_slice());

    let stdout = io::stdout();
    let mut write = stdout.lock();
    io::copy(&mut decompressor, &mut write)?;

    Ok(())
}

/// Print the log ouput of finished tasks.
/// Either print the logs of every task
/// or only print the logs of the specified tasks.
pub fn follow_task_logs(pueue_directory: String, task_id: usize, stderr: bool) {
    let (stdout_handle, stderr_handle) = match get_log_file_handles(task_id, &pueue_directory) {
        Ok((stdout, stderr)) => (stdout, stderr),
        Err(err) => {
            println!("Failed to get log file handles: {}", err);
            return;
        }
    };
    let mut handle = if stderr { stderr_handle } else { stdout_handle };

    let (out_path, err_path) = get_log_paths(task_id, &pueue_directory);
    let handle_path = if stderr { err_path } else { out_path };

    // Stdout handler to directly write log file output to io::stdout
    // without having to load anything into memory.
    let mut stdout = io::stdout();
    loop {
        // Check whether the file still exists. Exit if it doesn't.
        if !handle_path.exists() {
            println!("File has gone away. Did somebody remove the task?");
            return;
        }
        // Read the next chunk of text from the last position.
        if let Err(err) = io::copy(&mut handle, &mut stdout) {
            println!("Error while reading file: {}", err);
            return;
        };
    }
}
