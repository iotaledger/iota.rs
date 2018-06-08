pub const HASH_LENGTH: usize = 243;

pub trait ICurl
where
    Self: Default + Clone + Send + 'static,
{
    /// Absorb a `&[Trit]` into the sponge
    fn absorb(&mut self, trits: &[i32]);
    fn absorb_offset(&mut self, trits: &[i32], offset: usize, length: usize);
    /// Squeeze the sponge and writes to the provided output slice.
    fn squeeze(&mut self, out: &mut [i32]);
    fn squeeze_offset(&mut self, out: &mut [i32], offset: usize, length: usize);
    /// Exposes the complete state
    fn state(&self) -> &[i32];
    /// Exposes the complete mutable state
    fn state_mut(&mut self) -> &mut [i32];
}
