mod blood;
mod organ;
mod body;

use blood::Blood;
use organ::Organ;
use body::Body;

mod organs;
use organs::{Adipose, Liver, Marrow};

fn main() {
    let mut marrow = Marrow;
    let mut adipose = Adipose::new(1000.0);
    let blood = Blood::default();

    let blood = marrow.circulate(blood);
    println!("new_blood is {:?}", blood);
    let blood = adipose.circulate(blood);
    println!("new_blood is {:?}", blood);

    //
    // Body ([Organ, Organ])
    //
}
