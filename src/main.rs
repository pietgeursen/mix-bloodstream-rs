mod blood;
mod organ;
mod body;

use blood::Blood;
use organ::Organ;
use body::Body;

mod organs;
use organs::{Adipose, Liver, Marrow};

fn main() {
    let mut body = Body::new(
        Adipose::new(1000.0),
        Liver {},
        Marrow {}
    );

    body.pump();

    println!("my body working! {:?}", body.sugar())
}
