#[derive(Debug)]
pub struct Rad<T> {
    value: T,
}

impl Rad<u16> {
    pub fn new(value: u16) -> Self {
        Rad {
            value
        }
    }
    pub fn get_radians(&self) -> f32 {
        (self.value as f32) * 0.0001
    }
}

impl Rad<i16> {
    pub fn from_i16(value: i16) -> Self {
        Rad {
            value
        }
    }
    pub fn get_radians(&self) -> f32 {
        (self.value as f32) * 0.0001
    }
}
