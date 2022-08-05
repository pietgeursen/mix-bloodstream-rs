use crate::Organ;
use crate::{Adipose, Blood, Liver, Marrow};

pub struct Body {
    adipose: Adipose,
    liver: Liver,
    marrow: Option<Marrow>,
    blood: Blood,
}

impl Body {
    pub fn new(adipose: Adipose, liver: Liver, marrow: Option<Marrow>) -> Self {
        let blood = Blood::default();

        Self {
            adipose,
            liver,
            marrow,
            blood,
        }
    }

    pub fn sugar(&self) -> f32 {
        self.blood.sugar
    }

    pub fn pump(&mut self) {
        let blood = self.blood.clone();

        let blood = self.adipose.circulate(blood);
        let blood = self.liver.circulate(blood);

        // this worked, but we added let blood = self.blood.clone()
        // self.marrow.as_mut().map(|marrow| {
        //     self.blood = marrow.circulate(self.blood.clone());
        // });

        let blood = if let Some(marrow) = self.marrow.as_mut() {
            marrow.circulate(blood)
        } else {
            blood
        };

        self.blood = blood;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn body_pump() {
        let mut body = Body::new(Adipose::new(1000.0), Liver {}, Some(Marrow {}));
        assert_eq!(body.sugar(), 25.0);
        body.pump();
        assert_eq!(body.sugar(), 15.0);

        let mut body = Body::new(Adipose::new(1000.0), Liver {}, None);
        assert_eq!(body.sugar(), 25.0);
        body.pump();
        assert_eq!(body.sugar(), 20.0);
    }
}
