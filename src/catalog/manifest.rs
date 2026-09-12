use super::{Catalog, DuplicateExerciseError, Exercise};
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct CatalogManifest {
    pub schema_version: String,
    pub topics: Vec<ManifestTopic>,
}

#[derive(Debug, Deserialize)]
pub struct ManifestTopic {
    pub name: String,
    pub exercises: Vec<ManifestExercise>,
}

#[derive(Debug, Deserialize)]
pub struct ManifestExercise {
    pub name: String,
}

#[derive(Debug)]
pub enum ManifestError {
    Read(std::io::Error),
    Parse(toml::de::Error),
    UnsupportedVersion(String),
    TooManyTopics { attempted_topics: u16 },
    TooManyExercises { topic: u8, attempted_exercises: u16 },
    Catalog(DuplicateExerciseError),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ManifestError::Read(err) => write!(f, "{}", err),
            ManifestError::Parse(err) => write!(f, "{}", err),
            ManifestError::UnsupportedVersion(version) => {
                write!(f, "unsupported schema version: {}", version)
            }
            ManifestError::TooManyTopics {
                attempted_topics: count,
            } => write!(f, "too many topics: {}", count),
            ManifestError::TooManyExercises {
                topic,
                attempted_exercises: count,
            } => {
                write!(f, "too many exercises for topic {}: {}", topic, count)
            }
            ManifestError::Catalog(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ManifestError {}

impl Catalog {
    pub fn from_path(root: &Path) -> Result<Self, ManifestError> {
        let mut file =
            fs::File::open(root.join(".nixercises/catalog.toml")).map_err(ManifestError::Read)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(ManifestError::Read)?;
        let manifest: CatalogManifest = toml::from_str(&contents).map_err(ManifestError::Parse)?;
        if manifest.schema_version != "0.1" {
            return Err(ManifestError::UnsupportedVersion(manifest.schema_version));
        }
        let mut exercises = Vec::<Exercise>::new();
        let mut topic_id: u8 = 0;
        for topic in manifest.topics.iter() {
            let next_id = u16::from(topic_id) + 1;
            topic_id = u8::try_from(next_id).map_err(|_| ManifestError::TooManyTopics {
                attempted_topics: next_id,
            })?;
            let mut exercise_id: u8 = 0;
            for exercise in topic.exercises.iter() {
                let next_id = u16::from(exercise_id) + 1;
                exercise_id =
                    u8::try_from(next_id).map_err(|_| ManifestError::TooManyExercises {
                        topic: topic_id,
                        attempted_exercises: next_id,
                    })?;
                let exercise_path = root
                    .join("exercises")
                    .join(&topic.name)
                    .join(format!("{}.nix", exercise.name));
                exercises.push(Exercise::new(
                    topic_id,
                    exercise_id,
                    exercise.name.clone(),
                    exercise_path,
                ));
            }
        }
        let catalog = Catalog::new(exercises).map_err(ManifestError::Catalog)?;
        Ok(catalog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let root = Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/catalog/manifest/happy_path");
        let catalog = Catalog::from_path(&root).expect("valid manifest rejected");

        let expected = [
            ("t1e1", "exercise1", "topic1/exercise1.nix"),
            ("t1e2", "exercise2", "topic1/exercise2.nix"),
            ("t2e1", "exercise1", "topic2/exercise1.nix"),
        ];

        assert_eq!(catalog.len(), expected.len());

        for (exercise, (id, name, relative_path)) in catalog.iter().zip(expected) {
            assert_eq!(exercise.id().to_string(), id);
            assert_eq!(exercise.name(), name);

            let expected_path = root.join("exercises").join(relative_path);
            assert_eq!(exercise.path(), expected_path.as_path());
        }
    }

    #[test]
    fn missing_manifest() {
        let exercise_path = Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/catalog/manifest/missing_manifest");
        let catalog = Catalog::from_path(&exercise_path);
        assert!(
            matches!(catalog, Err(ManifestError::Read { .. })),
            "Expected Read error, got {:?}",
            catalog.err()
        )
    }

    #[test]
    fn malformed_toml() {
        let exercise_path = Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/catalog/manifest/malformed_toml");
        let catalog = Catalog::from_path(&exercise_path);
        assert!(
            matches!(catalog, Err(ManifestError::Parse { .. })),
            "Expected Parse error, got {:?}",
            catalog.err()
        )
    }

    #[test]
    fn unsupported_version() {
        let exercise_path = Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/catalog/manifest/unsupported_version");
        let catalog = Catalog::from_path(&exercise_path);
        assert!(
            matches!(catalog, Err(ManifestError::UnsupportedVersion { .. })),
            "Expected UnsupportedVersion error, got {:?}",
            catalog.err()
        )
    }

    #[test]
    fn too_many_topics() {
        let exercise_path = Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/catalog/manifest/too_many_topics");
        let catalog = Catalog::from_path(&exercise_path);
        assert!(
            matches!(catalog, Err(ManifestError::TooManyTopics { .. })),
            "Expected TooManyTopics error, got {:?}",
            catalog.err()
        )
    }

    #[test]
    fn too_many_exercises() {
        let exercise_path = Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/catalog/manifest/too_many_exercises");
        let catalog = Catalog::from_path(&exercise_path);
        assert!(
            matches!(catalog, Err(ManifestError::TooManyExercises { .. })),
            "Expected TooManyExercises error, got {:?}",
            catalog.err()
        )
    }
}
