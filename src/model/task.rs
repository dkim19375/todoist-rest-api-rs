//! Structures and enums representing objects in the Todoist Task API (<https://developer.todoist.com/rest/v2/?shell#tasks>)

use std::time::Duration;

use chrono::FixedOffset;
use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

/// A Todoist task
#[derive(Deserialize, Debug, Clone)]
pub struct Task {
    /// The task ID
    pub id: String,
    /// The [Project](crate::model::project::Project) ID that the task belongs to
    pub project_id: String,
    /// The [Section](crate::model::section::Section) ID that the task belongs to, [None] for tasks with no parent section
    pub section_id: Option<String>,
    /// The task content which may contain [markdown-formatted text and hyperlinks](https://todoist.com/help/articles/205195102)
    pub content: String,
    /// A description for the task which may contain [markdown-formatted text and hyperlinks](https://todoist.com/help/articles/205195102)
    pub description: String,
    /// Flag to mark completed task
    pub is_completed: bool,
    /// The task's labels (a list of names that may represent either personal or shared labels)
    pub labels: Vec<String>,
    /// ID of the parent task (read-only, will be [None] for top-level tasks)
    pub parent_id: Option<String>,
    /// The position under the same parent or project for top-level tasks (read-only)
    pub order: u32,
    /// The task's priority from 1 (normal, default value) to 4 (urgent)
    pub priority: u8,
    /// The task's due date or [None] if no date is set
    pub due: Option<TaskDueDateTime>,
    /// The URL to access this task in the Todoist web or mobile applications (read-only)
    pub url: String,
    /// The number of task comments (read-only)
    pub comment_count: u32,
    /// The date when the task was created (read-only)
    pub created_at: String,
    /// The ID of the user who created the task (read-only)
    pub creator_id: String,
    /// The responsible user ID (will be [None] if the task is unassigned)
    pub assignee_id: Option<String>,
    /// The ID of the user who assigned the task (read-only, will be [None] if the task is unassigned)
    pub assigner_id: Option<String>,
    /// The task's duration or [None] if the task has no duration
    pub duration: Option<TaskDuration>,
}

/// A structure that stores a task's due date and time
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDueDateTime {
    /// The human defined date in an arbitrary format
    pub string: String,
    /// The date in the format `YYYY-MM-DD` corrected to the user's timezone
    pub date: String,
    /// Whether the task has a [recurring due date](https://todoist.com/help/articles/360000636289)
    pub is_recurring: bool,
    /// Only returned if exact due time set (i.e. it's not a whole-day task),
    /// date and time in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format in UTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<String>,
    /// Only returned if exact due time set, user's timezone definition either in tzdata-compatible
    /// format ("Europe/Berlin") or as a string specifying east of the UTC offset as "UTC±HH:MM"
    /// (i.e. "UTC-01:00")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl TaskDueDateTime {
    /// Get the due date of a time as a `chrono` [DateTime]
    //#[cfg(feature = "chrono")]
    pub fn get_chrono_due_datetime(&self) -> Option<DateTime<FixedOffset>> {
        if let (Some(datetime), Some(timezone)) = (&self.datetime, &self.timezone) {
            let parsed = DateTime::parse_from_rfc3339(datetime).unwrap();
            if !timezone.starts_with("UTC") {
                let tz = timezone.parse::<Tz>().unwrap();
                let new_datetime = tz.from_local_datetime(&parsed.naive_utc()).unwrap();
                return Some(new_datetime.fixed_offset());
            }
            let chars = timezone.chars().collect::<Vec<char>>();
            let positive = chars[3] == '+';
            let hours = chars[4..6]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let minutes = chars[7..9]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let secs = hours * 3600 + minutes * 60;
            let fixed_offset = if positive {
                FixedOffset::east_opt(secs)
            } else {
                FixedOffset::west_opt(secs)
            }
            .unwrap();
            Some(
                fixed_offset
                    .from_local_datetime(&parsed.naive_utc())
                    .unwrap(),
            )
        } else {
            None
        }
    }
}

/// A structure storing the duration of a [Task]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDuration {
    /// The `amount` of time the task will take (greater than zero)
    pub amount: u64,
    /// The `unit` of time that the amount represents
    pub unit: TaskDurationUnit,
}

impl TaskDuration {
    /// Get the duration of a task as a [Duration]
    pub fn get_duration(&self) -> Duration {
        match self.unit {
            TaskDurationUnit::Minute => Duration::from_secs(self.amount * 60),
            TaskDurationUnit::Day => Duration::from_secs(self.amount * 60 * 60 * 24),
        }
    }

    /// Get the duration of a task as a [chrono::Duration]
    //#[cfg(feature = "chrono")]
    pub fn get_chrono_duration(&self) -> chrono::Duration {
        match self.unit {
            TaskDurationUnit::Minute => chrono::Duration::minutes(self.amount as i64),
            TaskDurationUnit::Day => chrono::Duration::days(self.amount as i64),
        }
    }
}

/// An enum representing the unit of time for a [TaskDuration]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskDurationUnit {
    /// A duration in minutes
    #[serde(rename = "minute")]
    Minute,
    /// A duration in days
    #[serde(rename = "day")]
    Day,
}
