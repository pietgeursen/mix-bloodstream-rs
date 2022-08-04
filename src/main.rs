mod blood;
mod organ;

mod adipose;
mod marrow;
mod liver;

use blood::Blood;
use organ::Organ;

use adipose::Adipose;
use marrow::Marrow;
use liver::Liver;


fn main() {
    let mut marrow = Marrow;
    let mut adipose = Adipose::new(1000.0);
    let blood = Blood::default();

    let blood = marrow.circulate(blood);
    println!("new_blood is {:?}", blood);
    let blood = adipose.circulate(blood);
    println!("new_blood is {:?}", blood);
}




