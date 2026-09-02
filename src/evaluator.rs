use std::path::PathBuf;
use std::process::{Command, ExitStatus};

pub enum EvalError {
    Command(std::io::Error),
    Evaluation {
        status: ExitStatus,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
    },
}

pub struct NixEvaluator {
    path: PathBuf,
    program: &'static str,
}

impl NixEvaluator {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            program: "nix",
        }
    }

    fn _new_with_program(path: PathBuf, program: &'static str) -> Self {
        Self { path, program }
    }

    pub fn evaluate(&self) -> Result<Vec<u8>, EvalError> {
        let output = Command::new(self.program)
            .args(["eval", "--file"])
            .arg(self.path.as_os_str())
            .output();
        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(result.stdout)
                } else {
                    Err(EvalError::Evaluation {
                        status: result.status,
                        stdout: result.stdout,
                        stderr: result.stderr,
                    })
                }
            }
            Err(command_error) => Err(EvalError::Command(command_error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nix_success() {
        let exercise_path = std::path::Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/evaluator/success.nix");
        let nix_evaluator = crate::evaluator::NixEvaluator::new(exercise_path.to_path_buf());
        let result = nix_evaluator.evaluate();
        assert![result.is_ok()];
        if let Ok(stdout) = result {
            assert_eq!(
                String::from_utf8_lossy(&stdout),
                "{ success = \"I succeeded!\"; }\n"
            );
        }
    }

    #[test]
    fn nix_failure() {
        let exercise_path = std::path::Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/evaluator/failure.nix");
        let nix_evaluator = crate::evaluator::NixEvaluator::new(exercise_path.to_path_buf());
        let result = nix_evaluator.evaluate();
        assert!(matches!(result, Err(EvalError::Evaluation { .. })));
        if let Err(EvalError::Evaluation {
            status,
            stdout: _,
            stderr,
        }) = result
        {
            assert!(!status.success());
            assert!(!stderr.is_empty());
        }
    }

    #[test]
    fn eval_failure() {
        let exercise_path = std::path::Path::new(env!["CARGO_MANIFEST_DIR"])
            .join("tests/fixtures/evaluator/eval_failure.nix");
        let nix_evaluator = crate::evaluator::NixEvaluator::_new_with_program(
            exercise_path.to_path_buf(),
            "not_nix",
        );
        let result = nix_evaluator.evaluate();
        assert!(matches!(result, Err(EvalError::Command { .. })));
        if let Err(EvalError::Command(err)) = result {
            assert!(err.kind() == std::io::ErrorKind::NotFound)
        }
    }
}
