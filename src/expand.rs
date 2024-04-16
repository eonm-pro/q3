use crate::Q3Error;

pub trait Expand
where
    Self: Sized,
{
    type State;

    fn expand(&mut self, state: Self::State) -> Result<Self::State, Q3Error>;
}
