pub const HASH_LENGTH: usize = 243;

pub trait ICurl
where
    Self: Default + Clone + Send + 'static,
{
    /// Absorb a `&[i8]` into the sponge
    fn absorb(&mut self, trits: &mut [i8]);
    /// Squeeze the sponge and writes to the provided output slice.
    fn squeeze(&mut self, out: &mut [i8]);
}
