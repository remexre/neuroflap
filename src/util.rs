use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

/// A wrapper for a single-thread shared RNG, using RefCell internally.
#[derive(Clone, Debug)]
pub struct SharedRng<R: Rng>(Rc<RefCell<R>>);

impl<R: Rng> SharedRng<R> {
    /// Creates a new instance of SharedRng.
    pub fn new(rng: R) -> SharedRng<R> {
        SharedRng(Rc::new(RefCell::new(rng)))
    }
}

impl<R: Rng> Rng for SharedRng<R> {
    fn next_u32(&mut self) -> u32 {
        let mut rng = self.0.borrow_mut();
        rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        let mut rng = self.0.borrow_mut();
        rng.next_u64()
    }

    fn next_f32(&mut self) -> f32 {
        let mut rng = self.0.borrow_mut();
        rng.next_f32()
    }

    fn next_f64(&mut self) -> f64 {
        let mut rng = self.0.borrow_mut();
        rng.next_f64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut rng = self.0.borrow_mut();
        rng.fill_bytes(dest)
    }
}
