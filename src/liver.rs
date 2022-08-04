use crate::Organ;
use crate::Blood;

pub struct Liver {
}

impl Liver {
    const CONSUME_RATE: f32 = 4.0;
    const PEPTIDE_DECAY_PROPORTION: f32 = 0.8;
}

impl Organ for Liver {
    fn circulate(&mut self, mut blood: Blood) -> Blood {
        blood.sugar -= Self::CONSUME_RATE;

        blood.glucagon = blood.glucagon * Self::PEPTIDE_DECAY_PROPORTION;
        blood.insulin = blood.insulin * Self::PEPTIDE_DECAY_PROPORTION;

        blood
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn circulate_blood_in_liver() {
        let mut liver = Liver{};
        let blood = Blood {
            sugar: 25.0,
            glucagon: 1.0,
            insulin: 5.0
        };
        let blood = liver.circulate(blood);
 
        assert_eq!(blood.sugar, 21.0);
        assert_eq!(blood.glucagon, 0.8);
        assert_eq!(blood.insulin, 4.0);
    }
}
