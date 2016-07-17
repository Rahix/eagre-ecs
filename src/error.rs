//! Error handling and Result type

use std::any;
use std::error;
use std::fmt;

use entity;

/// Error Type
#[derive(Debug, Clone, Copy)]
pub enum EcsError {
    /// The requested components was not found
    ComponentNotFound(any::TypeId),
    /// The requested entity was not found
    EntityNotFound(entity::Entity),
}

impl fmt::Display for EcsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EcsError::ComponentNotFound(ref err) => write!(f, "Component \"{:?}\" not found", err),
            EcsError::EntityNotFound(ref err) => write!(f, "Entity \"{}\" not found", err),
        }
    }
}

impl error::Error for EcsError {
    fn description(&self) -> &str {
        match *self {
            EcsError::ComponentNotFound(_) => "Component not found",
            EcsError::EntityNotFound(_) => "Entity not found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// Result Type
pub type EcsResult<T> = Result<T, EcsError>;
