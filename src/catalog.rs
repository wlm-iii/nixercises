use crate::exercise::Exercise;
use std::error::Error;
use std::fmt;
pub mod manifest;

pub struct Catalog {
    exercises: Vec<Exercise>,
}

#[derive(Debug)]
pub struct DuplicateExerciseError {
    first: Exercise,
    second: Exercise,
}

impl DuplicateExerciseError {
    fn new(first: Exercise, second: Exercise) -> Self {
        Self { first, second }
    }
}

impl fmt::Display for DuplicateExerciseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "duplicate exercise ID {} used by \"{}\" and \"{}\"",
            self.first.id(),
            self.first.name(),
            self.second.name()
        )
    }
}

impl Error for DuplicateExerciseError {}

impl Catalog {
    pub fn new(mut raw_exercises: Vec<Exercise>) -> Result<Self, DuplicateExerciseError> {
        raw_exercises.sort_by_key(|ex| *ex.id());
        if raw_exercises.len() > 1 {
            for e in 0..(raw_exercises.len() - 1) {
                if raw_exercises[e].cmp(&raw_exercises[e + 1]).is_eq() {
                    return Err(DuplicateExerciseError::new(
                        raw_exercises[e].clone(),
                        raw_exercises[e + 1].clone(),
                    ));
                }
            }
        }

        Ok(Self {
            exercises: raw_exercises,
        })
    }

    pub fn len(&self) -> usize {
        self.exercises.len()
    }

    pub fn is_empty(&self) -> bool {
        self.exercises.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Exercise> {
        self.exercises.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn populate() {
        let mut exercises = Vec::<Exercise>::new();
        for i in 1..3 {
            for j in 1..4 {
                exercises.push(Exercise::new(i, j, String::new(), PathBuf::new()));
            }
        }
        exercises.reverse();
        let catalog = Catalog::new(exercises).unwrap();
        assert!(!catalog.is_empty());
        for i in 0..catalog.len() - 1 {
            assert!(*catalog.exercises[i].id() < *catalog.exercises[i + 1].id())
        }
    }

    #[test]
    fn empty() {
        let catalog = Catalog::new(Vec::<Exercise>::new()).unwrap();
        assert!(catalog.is_empty());
        assert!(catalog.iter().next().is_none());
    }

    #[test]
    fn singular() {
        let exercises = vec![Exercise::new(1, 1, "single".to_string(), PathBuf::new())];
        let catalog = Catalog::new(exercises).unwrap();
        assert!(!catalog.is_empty());
        let mut catalog_iter = catalog.iter();
        assert!(catalog_iter.next().is_some());
        assert!(catalog_iter.next().is_none());
    }

    #[test]
    fn duplicate_id_only() {
        let exercises = vec![
            Exercise::new(1, 1, "foo".to_string(), PathBuf::new()),
            Exercise::new(1, 1, "bar".to_string(), PathBuf::new()),
        ];
        let catalog = Catalog::new(exercises);
        assert!(catalog.is_err());
        if let Err(dup_err) = catalog {
            assert_eq!(dup_err.first.id(), dup_err.second.id());
            assert_ne!(dup_err.first.name(), dup_err.second.name());
        }
    }

    #[test]
    fn duplicate_id_and_name() {
        let exercises = vec![
            Exercise::new(1, 1, "foo".to_string(), PathBuf::new()),
            Exercise::new(1, 1, "foo".to_string(), PathBuf::new()),
        ];
        let catalog = Catalog::new(exercises);
        assert!(catalog.is_err());
        if let Err(dup_err) = catalog {
            assert_eq!(dup_err.first.id(), dup_err.second.id());
            assert_eq!(dup_err.first.name(), dup_err.second.name());
        }
    }
}
