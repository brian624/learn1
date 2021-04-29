use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom}; // Include the `Error` type.

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

///refactored the list task display part here, so it can be reused
/// in the add and done tasks
fn display_task_list(task_list: Vec<Task>) -> Result<()> {
    // Enumerate and display tasks, if any.
    if task_list.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in task_list {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(tasks)
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&journal_path)?;

    // Consume file's contents as a vector of tasks.
    let mut tasks = collect_tasks(&file)?;

    // Try to remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    // Write the modified task list back into the file.
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    // added Enumerate and display tasks, if any.
    display_task_list(tasks)

    //Ok(())  removed in refactoring, result of display_task_list instead
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&journal_path)?;
    let mut tasks = collect_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    //Ok(())  -removed to add display of list after add
    display_task_list(tasks)
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new().read(true).open(&journal_path)?;
    // Parse the file and collect the tasks.
    let tasks = collect_tasks(&file)?;

    // refactored into the display_task_list function
    // Enumerate and display tasks, if any.
    //if tasks.is_empty() {
    //    println!("Task list is empty!");
    //} else {
    //    let mut order: u32 = 1;
    //    for task in tasks {
    //        println!("{}: {}", order, task);
    //       order += 1;
    //    }
    //}
    display_task_list(tasks)

    //Ok(())
}
