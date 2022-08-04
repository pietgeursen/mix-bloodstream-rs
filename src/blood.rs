#[derive(Debug)]
pub struct Blood {
    pub sugar: f32,
    pub glucagon: f32,
    pub insulin: f32,
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
