use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("This player is not currently in a game.")]
    NotInGame,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
