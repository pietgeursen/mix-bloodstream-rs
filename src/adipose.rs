use crate::{Blood, Organ};

pub struct Adipose {
    sugar: f32,
}

impl Adipose {
    const RELEASE_RATE: f32 = 1.0;
    const STORAGE_RATE: f32 = 1.0;

    pub fn new(sugar: f32) -> Self {
        Self { sugar }
    }
}

impl Organ for Adipose {
    fn circulate(&mut self, mut blood: Blood) -> Blood {
        if (blood.glucagon <= 0. && blood.insulin <= 0.0) || self.sugar <= 0.0 {
            return blood;
        }

        let sugar_released = self.sugar.min(blood.glucagon * Self::RELEASE_RATE);
        let sugar_stored = blood.sugar.min(blood.insulin * Self::STORAGE_RATE);

        blood.sugar += sugar_released - sugar_stored;
        self.sugar += sugar_stored - sugar_released;

        blood
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn circulate_blood_in_adipose() {
        let mut adipose = Adipose::new(1000.0);
        let blood = Blood::default();
        let blood = adipose.circulate(blood);

        assert_eq!(blood.sugar, 24.0);
    }
}
