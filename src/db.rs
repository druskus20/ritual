use color_eyre::eyre;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;
use crate::types::{DateTime, Day, Habit, HabitRef};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Db {
    path: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    pub days: HashMap<Uuid, Day>,
    pub habits: HashMap<Uuid, Habit>,
}

impl Db {
    #[tracing::instrument]
    pub fn open_or_new(path: PathBuf) -> Result<Self> {
        info!("Opening database at {:?}", path);
        if !path.exists() {
            info!("Database does not exist, creating a new one");
            // Create dir
            std::fs::create_dir_all(
                path.parent()
                    .expect("Invalid path, failed to get parent directory"),
            )?;
            let empty = State::default();
            let db = std::fs::File::create(&path)?;
            let writer = std::io::BufWriter::new(db);
            serde_json::to_writer(writer, &empty)?;
        }

        Ok(Self { path })
    }

    #[tracing::instrument]
    pub fn save(&self, state: &State) -> Result<()> {
        info!("Saving database to {:?}", self.path);
        let db = std::fs::File::create(&self.path)?;
        let writer = std::io::BufWriter::new(db);
        serde_json::to_writer(writer, state)?;
        Ok(())
    }

    #[tracing::instrument]
    pub fn load(&self) -> Result<State> {
        info!("Loading database from {:?}", self.path);
        let db = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(db);
        let state: State = serde_json::from_reader(reader)?;
        Ok(state)
    }
}

impl State {
    pub fn add_day(&mut self, date: DateTime) -> Result<()> {
        let day = Day::new(date);
        self.days
            .insert(day.id, day)
            .ok_or_else(|| eyre::eyre!("Failed to add day with date: {}", date))?;
        Ok(())
    }

    pub fn add_habit_to_day(&mut self, day_id: Uuid) -> Result<()> {
        let day = self
            .days
            .get_mut(&day_id)
            .ok_or_else(|| eyre::eyre!("Day not found"))?;
        let habit = Habit {
            id: Uuid::new_v4(),
            name: "New Habit".to_string(),
        };
        day.habits.insert(
            habit.id,
            HabitRef {
                id: habit.id,
                name: habit.name.clone(),
                done: false,
            },
        );
        self.habits.insert(habit.id, habit);
        Ok(())
    }

    pub fn set_habit_done(&mut self, day_id: Uuid, habit_id: Uuid, done: bool) -> Result<()> {
        let day = self
            .days
            .get_mut(&day_id)
            .ok_or_else(|| eyre::eyre!("Day not found"))?;
        let habit_ref = day
            .habits
            .get_mut(&habit_id)
            .ok_or_else(|| eyre::eyre!("Habit with ID {} not found in day {}", habit_id, day_id))?;

        habit_ref.done = done;
        Ok(())
    }
}
