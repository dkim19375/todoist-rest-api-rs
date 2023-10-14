//! Todoist Tasks API (<https://developer.todoist.com/rest/v2/?shell#tasks>)

use crate::internal::request::models::{CreateNewTaskArgs, UpdateTaskArgs};
use crate::internal::request::paths::create_path;
use crate::internal::request::{
    paths, send_todoist_delete_request, send_todoist_get_request, send_todoist_post_request,
    APIParametersError,
};
use crate::model::task::{Task, TaskDurationUnit};
use crate::todoist_config::TodoistConfig;
use crate::TodoistAPIError;

/// Get all active tasks
///
/// # Arguments
/// * `config` - The [TodoistConfig] used to use the Todoist API
/// * `project_id` - Filter the tasks by project ID
/// * `section_id` - Filter the tasks by section ID
/// * `label` - Filter the tasks by label name
/// * `filter` - Filter the tasks by any [supported filter](https://todoist.com/help/articles/205248842)
/// * `lang` - IETF language tag defining what language filter is written in, if differs from default English
/// * `ids` - Filter the tasks by a list of task IDs
///
/// # Precedence of parameters
/// When fetching a list of tasks, the API will do so in the following order:
/// * filter (with or without lang)
/// * ids
/// * label/project_id/section_id
//
// If you include a filter *and* IDs, only the filter will be used.
// If you include IDs and project_id, only IDs is used, and so on.
pub async fn get_active_tasks(
    config: &TodoistConfig,
    project_id: Option<String>,
    section_id: Option<String>,
    label: Option<String>,
    filter: Option<String>,
    lang: Option<String>,
    ids: Option<&[String]>,
) -> Result<Vec<Task>, TodoistAPIError> {
    let mut path_parts = vec![paths::TASKS.to_string()];
    if let Some(project_id) = project_id {
        path_parts.push(paths::PARAM_PROJECT_ID.into());
        path_parts.push(project_id);
    }
    if let Some(section_id) = section_id {
        path_parts.push(paths::PARAM_SECTION_ID.into());
        path_parts.push(section_id);
    }
    if let Some(label) = label {
        path_parts.push(paths::PARAM_LABEL.into());
        path_parts.push(label);
    }
    if let Some(filter) = filter {
        path_parts.push(paths::PARAM_FILTER.into());
        path_parts.push(filter);
    }
    if let Some(lang) = lang {
        path_parts.push(paths::PARAM_LANG.into());
        path_parts.push(lang);
    }
    if let Some(ids) = ids {
        if !ids.is_empty() {
            path_parts.push(paths::PARAM_IDS.into());
            path_parts.push(ids.join(","));
        }
    }
    send_todoist_get_request(config, create_path(&path_parts)).await
}

/// Create a new [Task]
///
/// Please note that only one of the `due_*` fields can be used at the same time (`due_lang` is a special case).
/// Also note that to remove the due date of a task completely, you should set the `due_string` parameter to `no date` or `no due date`.
///
/// # Arguments
/// * `config` - The [TodoistConfig] used to use the Todoist API
/// * `content` - The task content which may contain [markdown-formatted text and hyperlinks](https://todoist.com/help/articles/205195102)
/// * `description` - A description for the task which may contain [markdown-formatted text and hyperlinks](https://todoist.com/help/articles/205195102)
/// * `project_id` - The [Project](crate::model::project::Project) ID that the task belongs to, or the user's inbox if [None]
/// * `section_id` - The [Section](crate::model::section::Section) ID that the task belongs to
/// * `parent_id` - ID of the parent task
/// * `order` - The position under the same parent or project for top-level tasks
/// * `labels` - The task's labels (a list of names that may represent either personal or shared labels)
/// * `priority` - The task's priority from 1 (normal, default value) to 4 (urgent)
/// * `due_string` - A [human defined](https://todoist.com/help/articles/205325931) task due date (ex.: "next Monday", "Tomorrow").
/// Value is set using local (not UTC) time.
/// To remove the due date, you should set this to `no date` or `no due date`.
/// * `due_date` - Specific date in `YYYY-MM-DD` format relative to user's timezone
/// * `due_datetime` - Specific date and time in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format in UTC
/// * `due_lang` - 2-letter code specifying language in case `due_string` is not written in English
/// * `assignee_id` - The responsible user ID (only applies to shared tasks)
/// * `duration` - A positive (greater than zero) integer for the amount of `duration_unit` the task will take.
/// If specified, you **must** define a `duration_unit`
/// * `duration_unit` - The unit of time that the `duration` field above represents.
/// If specified, `duration` **must** be defined as well.
pub async fn create_new_task(
    config: &TodoistConfig,
    content: String,
    description: Option<String>,
    project_id: Option<String>,
    section_id: Option<String>,
    parent_id: Option<String>,
    order: Option<u32>,
    labels: Option<Vec<String>>,
    priority: Option<u8>,
    due_string: Option<String>,
    due_date: Option<String>,
    due_datetime: Option<String>,
    due_lang: Option<String>,
    assignee_id: Option<String>,
    duration: Option<u64>,
    duration_unit: Option<TaskDurationUnit>,
) -> Result<Task, TodoistAPIError> {
    validate_task_args(
        &due_string,
        &due_date,
        &due_datetime,
        &priority,
        &due_lang,
        &duration,
        &duration_unit,
    )?;

    send_todoist_post_request(
        config,
        paths::TASKS.to_string(),
        Some(&CreateNewTaskArgs {
            content,
            description,
            project_id,
            section_id,
            parent_id,
            order,
            labels,
            priority,
            due_string,
            due_date,
            due_datetime,
            due_lang,
            assignee_id,
            duration,
            duration_unit,
        }),
        true,
    )
    .await
}

/// Get an active (non-completed) [Task] by ID
pub async fn get_active_task(
    config: &TodoistConfig,
    task_id: String,
) -> Result<Task, TodoistAPIError> {
    send_todoist_get_request(config, get_task_path(task_id)).await
}

/// Create a new [Task]
///
/// Please note that only one of the `due_*` fields can be used at the same time (`due_lang` is a special case).
/// Also note that to remove the due date of a task completely, you should set the `due_string` parameter to `no date` or `no due date`.
///
/// # Arguments
/// * `config` - The [TodoistConfig] used to use the Todoist API
/// * `task_id` - The ID of the [Task] to update
/// * `content` - The task content which may contain [markdown-formatted text and hyperlinks](https://todoist.com/help/articles/205195102)
/// * `description` - A description for the task which may contain [markdown-formatted text and hyperlinks](https://todoist.com/help/articles/205195102)
/// * `labels` - The task's labels (a list of names that may represent either personal or shared labels)
/// * `priority` - The task's priority from 1 (normal, default value) to 4 (urgent)
/// * `due_string` - A [human defined](https://todoist.com/help/articles/205325931) task due date (ex.: "next Monday", "Tomorrow").
/// Value is set using local (not UTC) time.
/// To remove the due date, you should set this to `no date` or `no due date`.
/// * `due_date` - Specific date in `YYYY-MM-DD` format relative to user's timezone
/// * `due_datetime` - Specific date and time in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format in UTC
/// * `due_lang` - 2-letter code specifying language in case `due_string` is not written in English
/// * `assignee_id` - The responsible user ID (only applies to shared tasks)
/// * `duration` - A positive (greater than zero) integer for the amount of `duration_unit` the task will take.
/// If specified, you **must** define a `duration_unit`
/// * `duration_unit` - The unit of time that the `duration` field above represents.
/// If specified, `duration` **must** be defined as well.
pub async fn update_task(
    config: &TodoistConfig,
    task_id: String,
    content: Option<String>,
    description: Option<String>,
    labels: Option<Vec<String>>,
    priority: Option<u8>,
    due_string: Option<String>,
    due_date: Option<String>,
    due_datetime: Option<String>,
    due_lang: Option<String>,
    assignee_id: Option<String>,
    duration: Option<u64>,
    duration_unit: Option<TaskDurationUnit>,
) -> Result<Task, TodoistAPIError> {
    validate_task_args(
        &due_string,
        &due_date,
        &due_datetime,
        &priority,
        &due_lang,
        &duration,
        &duration_unit,
    )?;

    send_todoist_post_request(
        config,
        get_task_path(task_id),
        Some(&UpdateTaskArgs {
            content,
            description,
            labels,
            priority,
            due_string,
            due_date,
            due_datetime,
            due_lang,
            assignee_id,
            duration,
            duration_unit,
        }),
        true,
    )
    .await
}

/// Closes a [Task]
///
/// The command performs in the same way as the official clients:
/// * Regular tasks are marked complete and moved to history, along with their subtasks.
/// * Tasks with [recurring due dates](https://todoist.com/help/articles/360000636289) will be scheduled to their next occurrence.
pub async fn close_task(config: &TodoistConfig, task_id: String) -> Result<(), TodoistAPIError> {
    send_todoist_delete_request(config, create_path(&[paths::TASKS, &task_id, paths::CLOSE])).await
}

/// Reopens a [Task]
///
/// Any ancestor items or sections will also be marked as uncompleted and restored from history.
///
/// The reinstated items and sections will appear at the end of the list within their parent, after any previously active items.
pub async fn reopen_task(config: &TodoistConfig, task_id: String) -> Result<(), TodoistAPIError> {
    send_todoist_post_request::<(), ()>(
        config,
        create_path(&[paths::TASKS, &task_id, paths::REOPEN]),
        None,
        false,
    )
    .await
}

/// Deletes a [Task]
pub async fn delete_task(config: &TodoistConfig, task_id: String) -> Result<(), TodoistAPIError> {
    send_todoist_delete_request(config, get_task_path(task_id)).await
}

fn validate_task_args(
    due_string: &Option<String>,
    due_date: &Option<String>,
    due_datetime: &Option<String>,
    priority: &Option<u8>,
    due_lang: &Option<String>,
    duration: &Option<u64>,
    duration_unit: &Option<TaskDurationUnit>,
) -> Result<(), TodoistAPIError> {
    let due_types = vec![
        due_string.to_owned(),
        due_date.to_owned(),
        due_datetime.to_owned(),
    ]
    .iter()
    .filter_map(&Clone::clone)
    .collect::<Vec<String>>();
    if due_types.len() > 1 {
        return Err(APIParametersError {
            message: format!(
                "Only one of the due_* fields can be used at the same time, but {} were used: {}",
                due_types.len(),
                due_types.join(", ")
            ),
        }
        .into());
    }
    if let Some(priority) = priority {
        if *priority < 1 || *priority > 4 {
            return Err(APIParametersError {
                message: format!("The priority must be between 1 and 4 (was {})", priority),
            }
            .into());
        }
    }
    if let Some(due_date) = due_date.as_ref() {
        if let Some(error) = validate_yyyy_mm_dd(due_date.to_string()) {
            return Err(APIParametersError {
                message: format!(
                    "The due_date must be in format YYYY-MM-DD ({}, was {})",
                    error, due_date
                ),
            }
            .into());
        }
    }
    if let Some(due_datetime) = due_datetime.as_ref() {
        if let Some(error) = validate_rfc3339(due_datetime.to_string()) {
            return Err(APIParametersError {
                message: format!(
                    "The due_datetime must be in format RFC3339 ({}, was {})",
                    error, due_datetime
                ),
            }
            .into());
        }
    }
    if let Some(due_lang) = due_lang.as_ref() {
        if due_lang.len() != 2 {
            return Err(APIParametersError {
                message: format!("The due_lang must be a 2-letter code (was {})", due_lang),
            }
            .into());
        }
    }
    if let Some(duration) = duration {
        if duration_unit.is_none() {
            return Err(APIParametersError {
                message: format!("The duration_unit must be defined if duration is defined"),
            }
            .into());
        }
        if *duration == 0 {
            return Err(APIParametersError {
                message: format!("The duration must be greater than 0 (was {})", duration),
            }
            .into());
        }
    }
    if duration_unit.is_some() && duration.is_none() {
        return Err(APIParametersError {
            message: format!("The duration must be defined if duration_unit is defined"),
        }
        .into());
    }
    Ok(())
}

fn validate_yyyy_mm_dd(date: String) -> Option<String> {
    if date.len() != 10 {
        return Some("length was not 10".into());
    }
    if date.chars().nth(4).unwrap() != '-' || date.chars().nth(7).unwrap() != '-' {
        return Some("dashes missing".into());
    }
    if date[0..4].parse::<u16>().is_err() {
        return Some("invalid year".into());
    }
    if date[5..7].parse::<u8>().is_ok_and(|x| x >= 1 && x <= 12) {
        return Some("invalid month".into());
    }
    if date[8..10].parse::<u8>().is_ok_and(|x| x >= 1 && x <= 31) {
        return Some("invalid day".into());
    }
    None
}

fn validate_rfc3339(datetime: String) -> Option<String> {
    if datetime.len() < 20 {
        return Some("length was less than 20".into());
    }
    if let Some(error) = validate_yyyy_mm_dd(datetime[0..10].to_string()) {
        return Some(format!("invalid date: {}", error));
    }
    let t_or_space = datetime.chars().nth(10).unwrap().to_ascii_uppercase();
    if t_or_space != 'T' && t_or_space != ' ' {
        return Some("missing T or space".into());
    }
    if datetime[11..13]
        .parse::<u8>()
        .is_ok_and(|x| x >= 0 && x <= 23)
    {
        return Some("invalid hour".into());
    }
    if datetime[14..16]
        .parse::<u8>()
        .is_ok_and(|x| x >= 0 && x <= 59)
    {
        return Some("invalid minute".into());
    }
    if datetime[17..19]
        .parse::<u8>()
        .is_ok_and(|x| x >= 0 && x <= 60)
    {
        return Some("invalid second".into());
    }
    let mut timezone_index = 19;
    if datetime.chars().nth(19).unwrap() == '.' {
        if datetime.len() < 21 {
            return Some("Time cannot end with a decimal point '.'".into());
        }
        timezone_index += 1;
        for char in datetime[20..].chars() {
            if char == 'Z' || char == 'z' || char == '+' || char == '-' {
                if timezone_index == 20 {
                    return Some("Time cannot end with a decimal point '.'".into());
                }
                break;
            }
            timezone_index += 1;
            if char < '0' || char > '9' {
                return Some("invalid time fraction number".into());
            }
        }
    }
    let timezone_type = datetime.chars().nth(timezone_index).unwrap();
    if timezone_type == 'Z' || timezone_type == 'z' {
        if datetime.len() > timezone_index + 1 {
            return Some(format!(
                "Timezone '{}' must be the last character",
                timezone_type
            ));
        }
        return None;
    }
    if datetime[timezone_index + 1..timezone_index + 3]
        .parse::<u8>()
        .is_ok_and(|x| x >= 0 && x <= 23)
    {
        return Some("invalid timezone hour".into());
    }
    if datetime[timezone_index + 4..timezone_index + 6]
        .parse::<u8>()
        .is_ok_and(|x| x >= 0 && x <= 59)
    {
        return Some("invalid timezone minute".into());
    }
    None
}

fn get_task_path(task_id: String) -> String {
    create_path(&[paths::TASKS, &task_id])
}
