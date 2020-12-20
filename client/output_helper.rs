use std::collections::BTreeMap;
use std::io::stdout;

use crossterm::style::{style, Attribute, Color};
use crossterm::tty::IsTty;

use pueue::state::{GroupStatus, State};
use pueue::task::Task;

/// This is a simple small helper function with the purpose of easily styling text,
/// while also prevent styling if we're printing to a non-tty output.
/// If there's any kind of styling in the code, it should be done with the help of this function.
pub fn style_text<T: ToString>(
    text: T,
    color: Option<Color>,
    attribute: Option<Attribute>,
) -> String {
    let text = text.to_string();
    // No tty, we aren't allowed to do any styling
    if !stdout().is_tty() {
        return text;
    }

    let mut styled = style(text);
    if let Some(color) = color {
        styled = styled.with(color);
    }
    if let Some(attribute) = attribute {
        styled = styled.attribute(attribute);
    }

    styled.to_string()
}

pub fn has_special_columns(tasks: &BTreeMap<usize, Task>) -> (bool, bool) {
    // Check whether there are any delayed tasks.
    // In case there are, we need to add another column to the table.
    let has_delayed_tasks = tasks.iter().any(|(_id, task)| task.enqueue_at.is_some());

    // Check whether there are any tasks with dependencies.
    // In case there are, we need to add another column to the table.
    let has_dependencies = tasks
        .iter()
        .any(|(_id, task)| !task.dependencies.is_empty());

    (has_delayed_tasks, has_dependencies)
}

/// Return a nicely formatted headline that's displayed at the start of `pueue status`
pub fn get_default_headline(state: &State) -> String {
    // Print the current daemon state.
    let daemon_status_text = if state.running {
        style_text("running", Some(Color::Green), None)
    } else {
        style_text("paused", Some(Color::Yellow), None)
    };
    let parallel = state.settings.daemon.default_parallel_tasks;
    format!(
        "{} ({} parallel): {}",
        style_text("Default queue", None, Some(Attribute::Bold)),
        parallel,
        daemon_status_text
    )
}

/// Return a nicely formatted headline that's displayed above group tables
pub fn get_group_headline(name: &str, status: &GroupStatus, parallel: usize) -> String {
    // Style group name
    let name = style(format!("Group \"{}\"", name)).attribute(Attribute::Bold);

    // Print the current state of the group.
    let status = match status {
        GroupStatus::Running => style_text("running", Some(Color::Green), None),
        GroupStatus::Paused => style_text("paused", Some(Color::Yellow), None),
        GroupStatus::Reset => style_text("being reset", Some(Color::Red), None),
    };

    format!("{} ({} parallel): {}", name, parallel, status)
}

/// Get all tasks that aren't assigned to a group
/// Those tasks are displayed first.
pub fn get_default_tasks(tasks: &BTreeMap<usize, Task>) -> BTreeMap<usize, Task> {
    let mut default_tasks = BTreeMap::new();
    for (id, task) in tasks.iter() {
        if task.group.is_none() {
            default_tasks.insert(*id, task.clone());
        }
    }

    default_tasks
}

/// Sort given tasks by their groups
/// This is needed to print a table for each group
pub fn sort_tasks_by_group(
    tasks: &BTreeMap<usize, Task>,
) -> BTreeMap<String, BTreeMap<usize, Task>> {
    // We use a BTreeMap, since groups should be ordered alphabetically by their name
    let mut sorted_task_groups = BTreeMap::new();
    for (id, task) in tasks.iter() {
        if let Some(group) = &task.group {
            if !sorted_task_groups.contains_key(group) {
                sorted_task_groups.insert(group.clone(), BTreeMap::new());
            }
            sorted_task_groups
                .get_mut(group)
                .unwrap()
                .insert(*id, task.clone());
        }
    }

    sorted_task_groups
}
