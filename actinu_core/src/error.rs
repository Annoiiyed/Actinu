use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum ActinuError {
    #[snafu(display("Internal error: "))]
    Internal { reason: &'static str },

    #[snafu(display("Unknown error"))]
    Unknown,
}
