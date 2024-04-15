
pub trait Expand
where
    Self: Sized,
{
    type State;

    fn expand(&mut self, state: Self::State) -> Result<Self::State, Box<dyn std::error::Error>>;
}
