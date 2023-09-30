#[derive(Debug)]
pub struct Rad {
    value: f32,
}

impl Rad {
    pub fn new(value: u16) -> Self {
        Rad {
            value: f32::from(value),
        }
    }

    pub fn from_i16(value: i16) -> Self {
        Rad {
            value: f32::from(value),
        }
    }
    pub fn get_radians(&self) -> f32 {
        self.value * 0.0001
    }
}
