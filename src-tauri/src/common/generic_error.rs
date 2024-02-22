use std::sync::{PoisonError, MutexGuard};
use colored::*;

#[derive(Debug, thiserror::Error)]
pub enum GenericError {
    #[error(transparent)]
    Tauri(tauri::Error),

    #[error("Failed to acquire lock due to a poisoned mutex.")]
    MutexPoisonError,

    // Add a new variant for a custom error message
    #[error("ERROR in file {file} at line {line}: {message}")]
    Custom {
        message: String,
        file: String,
        line: u32,
    },
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for GenericError {
    fn from(_: PoisonError<MutexGuard<'_, T>>) -> Self {
        GenericError::MutexPoisonError
    }
}

impl serde::Serialize for GenericError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {   
        serializer.serialize_str(self.getErrorMessage().as_str())
    }
}

impl GenericError {
    pub fn new(message: &str, file: &str, line: &u32) -> GenericError {
        GenericError::Custom { message: message.to_string(), file: file.to_string(), line: *line }
    }

    pub fn getErrorMessage(&self) -> String {
        match self {
            GenericError::MutexPoisonError => {
                format!("Failed to acquire lock due to a poisoned mutex.")
            },
            GenericError::Custom { message, file, line } => {
                format!("ERROR in file {} at line {}: {}", file, line, message)
            },
            GenericError::Tauri(err) => {
              format!("Tauri error: {}", err)
            },
        }
    }

    pub fn print(&self) {
        println!("{}", self.getErrorMessage().red());
    }

    pub fn from<T, E: std::fmt::Debug>(result: Result<T, E>, message: &str, file: &str, line: &u32) -> Result<T, GenericError> {
        match result {
            Ok(value) => Ok(value),
            Err(_) => {
                let error = GenericError::new(message, file, line);
                Err(error)
            }
        }
    }
}
