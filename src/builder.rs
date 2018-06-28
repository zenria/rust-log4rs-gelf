use error::Error;

pub trait Builder {
    type TargetItem;
    
    fn build(self) -> Result<Self::TargetItem, Error>;
}