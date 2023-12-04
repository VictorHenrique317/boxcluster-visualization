use std::sync::{PoisonError, MutexGuard};

#[derive(Debug, thiserror::Error)]
pub enum GenericError {
    #[error(transparent)]
    Tauri(tauri::Error),

    #[error("Failed to acquire lock due to a poisoned mutex.")]
    MutexPoisonError,

    // Add a new variant for a custom error message
    #[error("{0}")]
    Custom(String),
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
        match self {
            GenericError::MutexPoisonError => {
                serializer.serialize_str("Failed to acquire lock due to a poisoned mutex.")
            },
            GenericError::Custom(message) => {
                serializer.serialize_str(message)
            },
            GenericError::Tauri(err) => {
              serializer.serialize_str(&format!("Tauri error: {}", err))
            },
        }
    }
}

impl GenericError {
    pub fn new(message: &str) -> GenericError {
        GenericError::Custom(message.to_string())
    }
}
