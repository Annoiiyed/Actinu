use crate::ActinuError;

pub trait Action<T> {
    fn execute(&self) -> Result<T, ActinuError>;
}
