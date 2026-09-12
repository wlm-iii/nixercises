use std::cmp::Ordering;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Exercise {
    id: ExerciseId,
    name: String,
    path: PathBuf,
}

impl Exercise {
    pub fn new(topic: u8, exercise: u8, name: String, path: PathBuf) -> Self {
        Self {
            id: ExerciseId::new(topic, exercise),
            name,
            path,
        }
    }

    pub fn id(&self) -> &ExerciseId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl PartialEq for Exercise {
    fn eq(&self, other: &Exercise) -> bool {
        self.id == other.id
    }
}

impl Eq for Exercise {}

impl PartialOrd for Exercise {
    fn partial_cmp(&self, other: &Exercise) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Exercise {
    fn cmp(&self, other: &Exercise) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl fmt::Display for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} ({})", self.id, self.name)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct ExerciseId {
    topic: u8,
    exercise: u8,
}

impl ExerciseId {
    fn new(topic: u8, exercise: u8) -> Self {
        Self { topic, exercise }
    }
}

impl fmt::Display for ExerciseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "t{}e{}", self.topic, self.exercise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn comparison() {
        let e1m1 = Exercise::new(1, 1, String::from("hangar"), PathBuf::new());
        let e1m2 = Exercise::new(1, 2, String::from("nuclear plant"), PathBuf::new());
        let e2m1 = Exercise::new(2, 1, String::from("deimos anomaly"), PathBuf::new());
        assert_eq!(e1m1.cmp(&e2m1), Ordering::Less);
        assert_eq!(e1m1.cmp(&e1m2), Ordering::Less);
        assert_eq!(e1m2.cmp(&e2m1), Ordering::Less);
        assert_eq!(
            e1m1,
            Exercise::new(1, 1, String::from("entryway"), PathBuf::new())
        );
        assert_eq!(
            e1m1,
            Exercise::new(1, 1, String::from("hangar"), PathBuf::new())
        );
    }

    #[test]
    fn display() {
        let e1m1_id = ExerciseId::new(1, 1);
        assert_eq!(e1m1_id.to_string(), "t1e1".to_string());
        let e1m1 = Exercise::new(1, 1, String::from("hangar"), PathBuf::new());
        assert_eq!(e1m1.to_string(), "t1e1 (hangar)".to_string());
    }
}
