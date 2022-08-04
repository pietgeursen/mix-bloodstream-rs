use std::fmt::Debug;

trait Organ {
    fn circulate(&mut self, blood: Blood) -> Blood;
}

struct Adipose {
    sugar: f32,
}

impl Adipose {
    const RELEASE_RATE: f32 = 1.0;
    const STORAGE_RATE: f32 = 1.0;
}

impl Organ for Adipose {
    fn circulate(&mut self, mut blood: Blood) -> Blood {
        if (blood.glucagon <= 0. && blood.insulin <= 0.0) || self.sugar <= 0.0 {
            return blood
        }

        let sugar_released = self.sugar.min(blood.glucagon * Self::RELEASE_RATE);
        let sugar_stored = blood.sugar.min(blood.insulin * Self::STORAGE_RATE);

        blood.sugar += sugar_released - sugar_stored;
        self.sugar += sugar_stored - sugar_released;

        blood
    }
}

#[derive(Debug)]
struct Blood {
    sugar: f32,
    glucagon: f32,
    insulin: f32,
}

impl Default for Blood {
    fn default() -> Self {
        Self {
            sugar: 25.0,
            glucagon: 0.0,
            insulin: 1.0,
        }
    }
}

struct Marrow;

impl Marrow {
    const SUGAR_CONSUMPTION: f32 = 5.0;
}

impl Organ for Marrow {
    fn circulate(&mut self, mut blood: Blood) -> Blood {
        blood.sugar = blood.sugar - Self::SUGAR_CONSUMPTION;
        blood
    }
}

fn main() {
    let mut marrow = Marrow;
    let mut adipose = Adipose{ sugar: 1000.0 };
    let blood = Blood::default();

    let blood = marrow.circulate(blood);
    println!("new_blood is {:?}", blood);
    let blood = adipose.circulate(blood);
    println!("new_blood is {:?}", blood);
}



#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn circulate_blood_in_adipose() {
        let mut adipose = Adipose{ sugar: 1000.0 };
        let blood = Blood::default();
        let blood = adipose.circulate(blood);
 
        assert_eq!(blood.sugar, 24.0);
    }
}
