use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;
use crate::types::{DateTime, Day, Habit};
use std::path::PathBuf;

pub struct Db {
    path: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    days: Vec<Day>,
    habits: Vec<Habit>,
}

impl Db {
    pub fn open_or_new(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            // Create dir
            std::fs::create_dir_all(&path)?;
            let empty = State::default();
            let db = std::fs::File::create(&path)?;
            let writer = std::io::BufWriter::new(db);
            serde_json::to_writer(writer, &empty)?;
        }

        Ok(Self { path })
    }

    pub fn save(&self, state: &State) -> Result<()> {
        let db = std::fs::File::create(&self.path)?;
        let writer = std::io::BufWriter::new(db);
        serde_json::to_writer(writer, state)?;
        Ok(())
    }

    fn load(&self) -> Result<State> {
        let db = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(db);
        let state: State = serde_json::from_reader(reader)?;
        Ok(state)
    }
}

impl State {
    pub async fn add_day(&mut self, date: DateTime) {
        self.days.push(Day::new(date));
    }
}
