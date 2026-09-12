use std::error::Error;
use std::process::ExitCode;
mod catalog;
mod evaluator;
mod exercise;

fn main() -> ExitCode {
    match run() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            eprintln!["Error: {}", err];
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let catalog = catalog::Catalog::from_path(&root)?;
    if !catalog.is_empty() {
        println!["{} exercises loaded...", catalog.len()];
        let mut passing = true;
        for exercise in catalog.iter() {
            let evaluator = evaluator::NixEvaluator::new(exercise.path().to_path_buf());
            println!["Results for {}:", exercise.name()];
            match evaluator.evaluate() {
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
                    println!["Unsuccessful error {}", String::from_utf8_lossy(&stderr)];
                    passing = false;
                }
                Err(evaluator::EvalError::Command(err)) => return Err(err.into()),
            }
        }
        if passing {
            println!["All exercises passing!"];
            return Ok(ExitCode::SUCCESS);
        } else {
            return Ok(ExitCode::FAILURE);
        }
    }
    println!["No exercises found"];
    Ok(ExitCode::SUCCESS)
}
