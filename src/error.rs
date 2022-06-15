use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("invalid skyeng data: {0}")]
    InvalidSkyengData(&'static str),
}
