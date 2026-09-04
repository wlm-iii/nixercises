use std::path::Path;
mod catalog;
mod evaluator;
mod exercise;

fn main() {
    let exercise_path = Path::new(env!["CARGO_MANIFEST_DIR"]).join("exercises/exercise.nix");
    let nix_evaluator = evaluator::NixEvaluator::new(exercise_path);

    match nix_evaluator.evaluate() {
        Ok(out) => {
            println!["Successful output: {}", String::from_utf8_lossy(&out)];
        }
        Err(evaluator::EvalError::Evaluation {
            status,
            stdout,
            stderr,
        }) => {
            println!["Unsuccessful output {}", String::from_utf8_lossy(&stdout)];
            if let Some(code) = status.code() {
                println!["Unsuccessful code {}", code];
            }
            println!["Unsuccessful error {}", String::from_utf8_lossy(&stderr)]
        }
        Err(evaluator::EvalError::Command(_)) => {
            println!["Some other error"]
        }
    }
}
