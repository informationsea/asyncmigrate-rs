use thiserror::Error;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("I/O Error")]
    IoError(#[from] std::io::Error),
    #[error("Format Error")]
    FormatError(#[from] std::fmt::Error),
    #[error("Utf8 Error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Parse Int Error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse Float Error")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("PostgreSQL Error")]
    PostgresError(#[from] tokio_postgres::Error),
    #[error("{0}: V{1}")]
    InconsistentMigrationError(&'static str, i32),
    #[error("Version mismatch: local version: {0} database version: {1}")]
    VersionMismatchError(i32, i32),
    #[error("Error: {0}")]
    OtherError(&'static str),
}
