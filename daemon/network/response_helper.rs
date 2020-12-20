use pueue::{network::message::create_failure_message, task::TaskStatus};
use pueue::{network::message::Message, state::SharedState};

/// Check whether the given group exists. Return an failure message if it doesn't.
pub fn ensure_group_exists(state: &SharedState, group: &str) -> Result<(), Message> {
    let state = state.lock().unwrap();
    if !state.groups.contains_key(group) {
        return Err(create_failure_message(format!(
            "Group {} doesn't exists. Use one of these: {:?}",
            group,
            state.groups.keys()
        )));
    }

    Ok(())
}

pub fn task_response_helper(
    message: &str,
    task_ids: Vec<usize>,
    statuses: Vec<TaskStatus>,
    state: &SharedState,
) -> String {
    // Get all matching/mismatching task_ids for all given ids and statuses.
    let (matching, mismatching) = {
        let mut state = state.lock().unwrap();
        state.tasks_in_statuses(statuses, Some(task_ids))
    };

    compile_task_response(message, matching, mismatching)
}

/// Compile a response for instructions with multiple tasks ids
/// A custom message will be combined with a text about all matching tasks
/// and possibly tasks for which the instruction cannot be executed.
pub fn compile_task_response(
    message: &str,
    matching: Vec<usize>,
    mismatching: Vec<usize>,
) -> String {
    let matching: Vec<String> = matching.iter().map(|id| id.to_string()).collect();
    let mismatching: Vec<String> = mismatching.iter().map(|id| id.to_string()).collect();
    let matching_string = matching.join(", ");

    // We don't have any mismatching ids, return the simple message.
    if mismatching.is_empty() {
        return format!("{}: {}", message, matching_string);
    }

    let mismatched_message = "The command failed for tasks";
    let mismatching_string = mismatching.join(", ");

    // All given ids are invalid.
    if matching.is_empty() {
        return format!("{}: {}", mismatched_message, mismatching_string);
    }

    // Some ids were valid, some were invalid.
    format!(
        "{}: {}\n{}: {}",
        message, matching_string, mismatched_message, mismatching_string
    )
}
