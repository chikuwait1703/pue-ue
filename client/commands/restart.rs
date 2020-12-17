use anyhow::{bail, Result};

use pueue::message::*;
use pueue::protocol::*;
use pueue::task::{Task, TaskStatus};

use crate::commands::edit::edit_line;
use crate::commands::get_state;

/// When Restarting tasks, the remote state is queried and a AddMessage
/// is create from the existing task in the state.
///
/// This is done on the client-side, so we can easily edit the task before restarting it.
pub async fn restart(
    socket: &mut GenericStream,
    task_ids: Vec<usize>,
    start_immediately: bool,
    stashed: bool,
    edit_command: bool,
    edit_path: bool,
    in_place: bool,
) -> Result<()> {
    let new_status = if stashed {
        TaskStatus::Stashed
    } else {
        TaskStatus::Queued
    };

    let mut state = get_state(socket).await?;
    let (matching, mismatching) = state.tasks_in_statuses(vec![TaskStatus::Done], Some(task_ids));

    // Build a RestartMessage, if the tasks should be replaced instead of creating a copy of the
    // original task. This is only important, if replace is `True`.
    let mut restart_message = RestartMessage {
        tasks: Vec::new(),
        stashed,
        start_immediately,
    };

    // Go through all Done commands we found and restart them
    for task_id in &matching {
        let task = state.tasks.get(task_id).unwrap();
        let mut new_task = Task::from_task(task);
        new_task.status = new_status.clone();

        // Path and command can be edited, if the use specified the -e or -p flag.
        let mut command = task.original_command.clone();
        let mut path = task.path.clone();
        if edit_command {
            command = edit_line(&command)?
        };
        if edit_path {
            path = edit_line(&path)?;
        }

        // Add the tasks to the singular message, if we want to restart the tasks in-place.
        // And continue with the next task. The message will then be sent after the for loop.
        if in_place {
            restart_message.tasks.push(TasksToRestart {
                task_id: *task_id,
                command,
                path,
            });

            continue;
        }

        // Create a AddMessage to send the task to the daemon from the updated info and the old task.
        let add_task_message = Message::Add(AddMessage {
            command,
            path,
            envs: task.envs.clone(),
            start_immediately,
            stashed,
            group: task.group.clone(),
            enqueue_at: None,
            dependencies: Vec::new(),
            print_task_id: false,
        });

        // Send the cloned task to the daemon and abort on any failure messages.
        send_message(add_task_message, socket).await?;
        if let Message::Failure(message) = receive_message(socket).await? {
            bail!(message);
        };
    }

    // Send the singular in-place restart message to the daemon.
    if in_place {
        send_message(Message::Restart(restart_message), socket).await?;
        if let Message::Failure(message) = receive_message(socket).await? {
            bail!(message);
        };
    }

    if !matching.is_empty() {
        println!("Restarted tasks: {:?}", matching);
    }
    if !mismatching.is_empty() {
        println!("Couldn't restart tasks: {:?}", mismatching);
    }

    Ok(())
}
