use crate::Blood;

pub trait Organ {
    fn circulate(&mut self, blood: Blood) -> Blood;
}
