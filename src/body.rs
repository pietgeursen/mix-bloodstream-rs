// use crate::Organ;
use crate::{Adipose, Liver, Marrow, Blood};
use crate::Organ;

pub struct Body {
    // organs: [dyn Organ]
    adipose: Adipose,
    liver: Liver,
    marrow: Marrow,
    blood: Blood
}

impl Body {
    pub fn new (adipose: Adipose, liver: Liver, marrow: Marrow) -> Self {
        let blood = Blood::default();

        Self {
            adipose,
            liver,
            marrow,
            blood
        }
    }

    pub fn sugar (&self) -> f32 {
        self.blood.sugar
    }

    pub fn pump (&mut self) {
        self.blood = self.adipose.circulate(self.blood.clone());
        self.blood = self.liver.circulate(self.blood.clone());
        self.blood = self.marrow.circulate(self.blood.clone());
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn body_pump () {
        let adipose = Adipose::new(1000.0);
        let liver = Liver {};
        let marrow = Marrow {};

        let mut body = Body::new(adipose, liver, marrow);
            
        assert_eq!(body.sugar(), 25.0);
        body.pump();
        assert_eq!(body.sugar(), 15.0);
    }
}
