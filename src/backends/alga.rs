#![cfg(feature="alga")]

use crate::group::Group;

extern crate alga;
use alga::general::{AbstractGroup, Multiplicative};

impl<G: AbstractGroup<Multiplicative>> Group for G {
    fn identity() -> Self {
        G::identity()
    }
    fn inverse(&self) -> Self {
        self.two_sided_inverse()
    }
    fn compose(&self, other: &Self) -> Self {
        self.operate(other)
    }
}
