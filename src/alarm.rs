use std::{fs::File, io::Read, path::PathBuf};

use flagset::{flags, FlagSet};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use time::Time;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alarm {
    name: String,
    time: Time,
    repeat: RepeatType,
    processes: Vec<Process>,
}

impl Alarm {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RepeatType {
    Daily,
    Weekly,
    Days(DaySet),
}

flags! {
    pub enum Days: u8 {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DaySet(FlagSet<Days>);

impl DaySet {
    pub fn new(days: impl Into<FlagSet<Days>>) -> Self {
        DaySet(days.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Process {
    Name(String),
    Exec(PathBuf),
}

impl Process {
    pub fn from_name(name: impl Into<String>) -> Self {
        Self::Name(name.into())
    }

    pub fn from_exec(path: impl Into<PathBuf>) -> Self {
        Self::Exec(path.into())
    }
}

pub fn read_alarms(file: &mut File) -> Result<Vec<Alarm>, Box<dyn std::error::Error>> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.is_empty() || contents == "[]" {
        return Ok(Vec::new());
    }

    let alarms: Vec<Alarm> = serde_json::from_str(&contents)?;
    Ok(alarms)
}
