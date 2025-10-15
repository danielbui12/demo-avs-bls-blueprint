use std::net::AddrParseError;

use blueprint_sdk::eigensdk::{
    types::operator::OperatorTypesError,
};

#[expect(clippy::large_enum_variant, reason = "SDK error is large currently")]
#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Aggregation: {0}")]
    Aggregation(String),
    #[error(transparent)]
    OperatorTypesError(#[from] OperatorTypesError),
    #[error("Context: {0}")]
    Context(String),
    #[error(transparent)]
    Parse(#[from] AddrParseError),
    #[error("Runtime: {0}")]
    Runtime(String),
    #[error("Task: {0}")]
    Task(String),
}