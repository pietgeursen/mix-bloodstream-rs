use crate::{Blood, Organ};

pub struct Marrow;

impl Marrow {
    const SUGAR_CONSUMPTION: f32 = 5.0;
}

impl Organ for Marrow {
    fn circulate(&mut self, mut blood: Blood) -> Blood {
        blood.sugar = blood.sugar - Self::SUGAR_CONSUMPTION;
        blood
    }
}
