use error::Error;

pub trait Builder {
    type TargetItem;

    fn new() -> Self;
    fn build(self) -> Result<Self::TargetItem, Error>;
}