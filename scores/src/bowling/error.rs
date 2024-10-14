#[derive(Debug, Default, PartialEq)]
pub enum Error {
    #[default] None,
    InvalidFrame,
    GameAlreadyEnded,
}
