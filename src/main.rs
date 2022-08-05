mod blood;
mod body;
mod organ;

use blood::Blood;
use body::Body;
use organ::Organ;

mod organs;
use organs::{Adipose, Liver, Marrow};

fn main() {
    let mut body = Body::new(Adipose::new(1000.0), Liver {}, Some(Marrow {}));

    body.pump();

    println!("my body working! {:?}", body.sugar())
}
